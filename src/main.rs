// This is a portfolio project so to anyone reading this, I would appreciate it if you posted some issues and gave me feedback on how I could improve this code.
// N.B SDL3 was used because it is an API I am most familiar with even with rust.
// Serde documentation: https://serde.rs/
// I tried to be as clean as possible and avoid unwrap() where possible to exercise my rust skills and program design.
// Chip8 tutorial used : https://github.com/orcalinux/chip8-emulator, https://tobiasvl.github.io/blog/write-a-chip-8-emulator/ and https://www.freecodecamp.org/news/creating-your-very-own-chip-8-emulator/
// Cross referencing really helped me understand the implementation better.
// My understanding of the Chip8 architecture and the SDL3 API was greatly improved by these resources.

use gtk::prelude::*;
use gtk::{
    Application, ApplicationWindow, Box as GtkBox, Button, CheckButton, DropDown,
    FileChooserAction, FileChooserDialog, HeaderBar, Label, Orientation, ResponseType,
};
use gtk4 as gtk;
use sdl3::event::Event;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

mod cpu;
use cpu::CPU;

mod audio;
use audio::Audio;

mod window;
use window::Window;

mod input;
use input::InputHandler;

mod util;

// For my benefit, I used GTK4 because I got to use GTK3 after following a tutorial from Rust Programming by Example by Packt.
// It is more modernt than GKT3

const WINDOW_WIDTH: i32 = 400;
const WINDOW_HEIGHT: i32 = 500;
const CPU_CYCLES_PER_FRAME: u32 = 8; // ~500Hz at 60Hz frame rate

// Structure to store recent ROMs
#[derive(Serialize, Deserialize)]
struct RecentRoms {
    roms: Vec<String>,
}

fn main() {
    // Initialize GTK
    let app = Application::builder()
        .application_id("com.euclidae.chip8_emulator")
        .build();

    app.connect_activate(|app| {
        // Create the main window
        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(WINDOW_WIDTH)
            .default_height(WINDOW_HEIGHT)
            .build();

        // Create header bar
        let header_bar = HeaderBar::builder()
            .title_widget(&Label::new(Some("Euclidae's CHIP-8 Emulator")))
            .build();

        // Create main vertical box
        let vbox = GtkBox::new(Orientation::Vertical, 10); // This was used in toolbar.rs in Tuneup, when you were first learning how to make a music player with GTK3 from the above book.
        vbox.set_margin_top(20);
        vbox.set_margin_bottom(20);
        vbox.set_margin_start(20);
        vbox.set_margin_end(20);
        vbox.set_halign(gtk::Align::Center);
        vbox.set_valign(gtk::Align::Start);

        // Status label
        let status_label = Label::new(Some("Select a ROM to start"));
        status_label.set_margin_bottom(10);
        vbox.append(&status_label);

        // Load recent ROMs
        let recent_roms = load_recent_roms().unwrap_or(RecentRoms { roms: vec![] });
        let recent_dropdown = DropDown::from_strings(
            &recent_roms
                .roms
                .iter()
                .map(|s| s.as_str())
                .collect::<Vec<_>>(),
        );
        recent_dropdown.set_sensitive(!recent_roms.roms.is_empty());
        recent_dropdown.set_margin_bottom(10);

        // Resolution scaler dropdown
        let scale_dropdown =
            DropDown::from_strings(&["8x (512x256)", "10x (640x320)", "12x (768x384)"]);
        scale_dropdown.set_selected(0);

        // Clone widgets for closures
        let status_label_clone1 = status_label.clone();
        let window_clone1 = window.clone();
        let scale_dropdown_clone1 = scale_dropdown.clone();
        recent_dropdown.connect_selected_item_notify(move |dropdown| {
            if let Some(item) = dropdown.selected_item() {
                let path = item.downcast_ref::<gtk::StringObject>().unwrap().string();
                let scale = match scale_dropdown_clone1.selected() {
                    0 => 8,
                    1 => 10,
                    2 => 12,
                    _ => 8,
                };
                window_clone1.hide();
                match start_emulator(PathBuf::from(path.as_str()), scale, true) {
                    Ok(_) => {
                        status_label_clone1.set_text("Emulator closed successfully");
                        window_clone1.show();
                    }
                    Err(e) => {
                        status_label_clone1.set_text(&format!("Emulator error: {}", e));
                        window_clone1.show();
                    }
                }
            }
        });
        vbox.append(&recent_dropdown);

        // Select ROM button
        let select_button = Button::builder()
            .label("Select CHIP-8 ROM")
            .halign(gtk::Align::Center)
            .build();
        select_button.add_css_class("suggested-action");
        let status_label_clone2 = status_label.clone();
        let recent_dropdown_clone2 = recent_dropdown.clone();
        let window_clone2 = window.clone();
        let scale_dropdown_clone2 = scale_dropdown.clone();
        select_button.connect_clicked(move |_| {
            let dialog = FileChooserDialog::new(
                Some("Select a CHIP-8 ROM"),
                Some(&window_clone2),
                FileChooserAction::Open,
                &[
                    ("Open", ResponseType::Accept),
                    ("Cancel", ResponseType::Cancel),
                ],
            );

            let filter = gtk::FileFilter::new();
            filter.add_pattern("*.ch8");
            filter.set_name(Some("CHIP-8 ROM files (*.ch8)"));
            dialog.add_filter(&filter);

            let status_label_clone3 = status_label_clone2.clone();
            let recent_dropdown_clone3 = recent_dropdown_clone2.clone();
            let window_clone3 = window_clone2.clone();
            let scale_dropdown_clone3 = scale_dropdown_clone2.clone();
            dialog.connect_response(move |dialog, response| {
                if response == ResponseType::Accept {
                    if let Some(file) = dialog.file() {
                        if let Some(path) = file.path() {
                            let scale = match scale_dropdown_clone3.selected() {
                                0 => 8,
                                1 => 10,
                                2 => 12,
                                _ => 8,
                            };
                            window_clone3.hide();
                            match start_emulator(path.clone(), scale, true) {
                                Ok(_) => {
                                    status_label_clone3.set_text("Emulator closed successfully");
                                    add_recent_rom(
                                        path.to_string_lossy().to_string(),
                                        &recent_dropdown_clone3,
                                    );
                                    window_clone3.show();
                                }
                                Err(e) => {
                                    status_label_clone3.set_text(&format!("Emulator error: {}", e));
                                    window_clone3.show();
                                }
                            }
                        }
                    }
                }
                dialog.close();
            });

            dialog.show();
        });
        vbox.append(&select_button);

        // Resolution scaler dropdown
        let scale_label = Label::new(Some("Display Scale:"));
        scale_label.set_margin_top(10);
        vbox.append(&scale_label);
        vbox.append(&scale_dropdown);

        // Audio toggle
        let audio_toggle = CheckButton::with_label("Enable Audio");
        audio_toggle.set_active(true);
        audio_toggle.set_margin_top(10);
        vbox.append(&audio_toggle);

        // Theme switcher
        let theme_button = Button::builder()
            .label("Toggle Dark/Light Theme")
            .halign(gtk::Align::Center)
            .build();
        theme_button.add_css_class("suggested-action");
        theme_button.set_margin_top(10);
        let is_dark = std::cell::RefCell::new(true);
        theme_button.connect_clicked(move |_| {
            let mut is_dark = is_dark.borrow_mut();
            *is_dark = !*is_dark;
            let settings = gtk::Settings::default().unwrap();
            settings.set_gtk_application_prefer_dark_theme(*is_dark);
        });
        vbox.append(&theme_button);

        // Reset recent ROMs
        let reset_button = Button::builder()
            .label("Clear Recent ROMs")
            .halign(gtk::Align::Center)
            .build();
        reset_button.add_css_class("destructive-action");
        reset_button.set_margin_top(10);
        let recent_dropdown_clone4 = recent_dropdown.clone();
        let status_label_clone4 = status_label.clone();
        reset_button.connect_clicked(move |_| {
            let _ = fs::write(
                "recent_roms.json",
                serde_json::to_string(&RecentRoms { roms: vec![] }).unwrap(),
            );
            recent_dropdown_clone4.set_model(Some(&gtk::StringList::new(&[])));
            recent_dropdown_clone4.set_sensitive(false);
            status_label_clone4.set_text("Recent ROMs cleared");
        });
        vbox.append(&reset_button);

        window.set_titlebar(Some(&header_bar));
        window.set_child(Some(&vbox));
        window.show();
    });

    app.run();
}

fn start_emulator(rom_path: PathBuf, scale: u32, enable_audio: bool) -> Result<(), String> {
    // Initialize SDL
    let sdl = sdl3::init().map_err(|e| e.to_string())?;

    // Create SDL window
    let rom_name = rom_path.file_name().unwrap_or_default().to_string_lossy();
    let win = Window::new(&format!("Euclidae's CHIP-8: {}", rom_name), scale)?;

    // Initialize audio
    let audio = if enable_audio {
        Audio::new()?
    } else {
        Audio::new_silent()
    };

    // Read ROM file
    let rom = fs::read(&rom_path).map_err(|e| e.to_string())?;

    // Initialize CPU
    let mut cpu = CPU::new(win, audio);
    cpu.load_rom(&rom)?;

    // Initialize input handler with shared running flag
    let running = Arc::new(Mutex::new(true));
    let mut input_handler = InputHandler::new(&sdl, Arc::clone(&running))?;

    // Main loop
    let mut last_frame_time = std::time::Instant::now();
    while *running.lock().unwrap() {
        for event in input_handler.poll_events() {
            match event {
                Event::Quit { .. } => {
                    *running.lock().unwrap() = false;
                    cpu.audio.pause();
                }
                _ => {}
            }
        }

        input_handler.update();
        for _ in 0..CPU_CYCLES_PER_FRAME {
            cpu.run_loop(&input_handler)?;
        }

        let frame_time = last_frame_time.elapsed();
        if frame_time < std::time::Duration::from_millis(16) {
            std::thread::sleep(std::time::Duration::from_millis(16) - frame_time);
        }
        last_frame_time = std::time::Instant::now();
    }

    Ok(())
}

fn load_recent_roms() -> Result<RecentRoms, String> {
    let data = fs::read_to_string("recent_roms.json").map_err(|e| e.to_string())?;
    let roms: RecentRoms = serde_json::from_str(&data).map_err(|e| e.to_string())?;
    Ok(roms)
}

fn add_recent_rom(path: String, dropdown: &DropDown) {
    let mut recent_roms = load_recent_roms().unwrap_or(RecentRoms { roms: vec![] });
    recent_roms.roms.retain(|p| p != &path);
    recent_roms.roms.insert(0, path);
    if recent_roms.roms.len() > 5 {
        recent_roms.roms.pop();
    }
    let _ = fs::write(
        "recent_roms.json",
        serde_json::to_string(&recent_roms).unwrap(),
    );
    dropdown.set_model(Some(&gtk::StringList::new(
        &recent_roms
            .roms
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<_>>(),
    )));
    dropdown.set_sensitive(true);
}

/*MIT License

Copyright (c) 2025 Euclidae

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE. */
