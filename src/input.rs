// This was too long so it got its own module.
use sdl3::EventPump;
use sdl3::Sdl;
use sdl3::keyboard::Scancode;
use std::sync::{Arc, Mutex};

pub struct InputHandler {
    event_pump: EventPump,
    keys: [bool; 16],
    running: Arc<Mutex<bool>>,
}

impl InputHandler {
    pub fn new(sdl: &Sdl, running: Arc<Mutex<bool>>) -> Result<InputHandler, String> {
        let event_pump = sdl.event_pump().map_err(|e| e.to_string())?;
        println!("InputHandler initialized");
        Ok(InputHandler {
            event_pump,
            keys: [false; 16],
            running,
        })
    }

    pub fn poll_events(&mut self) -> impl Iterator<Item = sdl3::event::Event> {
        self.event_pump.pump_events();
        self.event_pump.poll_iter()
    }

    pub fn update(&mut self) {
        let keyboard_state = self.event_pump.keyboard_state();
        self.keys = [false; 16];

        if keyboard_state.is_scancode_pressed(Scancode::Kp1) {
            self.keys[0x1] = true;
            println!("Key 1 (Kp1) pressed");
        }
        if keyboard_state.is_scancode_pressed(Scancode::Kp2) {
            self.keys[0x2] = true;
            println!("Key 2 (Kp2) pressed");
        }
        if keyboard_state.is_scancode_pressed(Scancode::Kp3) {
            self.keys[0x3] = true;
            println!("Key 3 (Kp3) pressed");
        }
        if keyboard_state.is_scancode_pressed(Scancode::Kp4) {
            self.keys[0xc] = true;
            println!("Key C (Kp4) pressed");
        }
        if keyboard_state.is_scancode_pressed(Scancode::Q) {
            self.keys[0x4] = true;
            println!("Key 4 (Q) pressed");
        }
        if keyboard_state.is_scancode_pressed(Scancode::W) {
            self.keys[0x5] = true;
            println!("Key 5 (W) pressed");
        }
        if keyboard_state.is_scancode_pressed(Scancode::E) {
            self.keys[0x6] = true;
            println!("Key 6 (E) pressed");
        }
        if keyboard_state.is_scancode_pressed(Scancode::R) {
            self.keys[0xd] = true;
            println!("Key D (R) pressed");
        }
        if keyboard_state.is_scancode_pressed(Scancode::A) {
            self.keys[0x7] = true;
            println!("Key 7 (A) pressed");
        }
        if keyboard_state.is_scancode_pressed(Scancode::S) {
            self.keys[0x8] = true;
            println!("Key 8 (S) pressed");
        }
        if keyboard_state.is_scancode_pressed(Scancode::D) {
            self.keys[0x9] = true;
            println!("Key 9 (D) pressed");
        }
        if keyboard_state.is_scancode_pressed(Scancode::F) {
            self.keys[0xe] = true;
            println!("Key E (F) pressed");
        }
        if keyboard_state.is_scancode_pressed(Scancode::Z) {
            self.keys[0xa] = true;
            println!("Key A (Z) pressed");
        }
        if keyboard_state.is_scancode_pressed(Scancode::X) {
            self.keys[0x0] = true;
            println!("Key 0 (X) pressed");
        }
        if keyboard_state.is_scancode_pressed(Scancode::C) {
            self.keys[0xb] = true;
            println!("Key B (C) pressed");
        }
        if keyboard_state.is_scancode_pressed(Scancode::V) {
            self.keys[0xf] = true;
            println!("Key F (V) pressed");
        }
        if keyboard_state.is_scancode_pressed(Scancode::Slash) {
            self.keys[0xb] = true;
            println!("Key B (Slash) pressed");
        }
        if keyboard_state.is_scancode_pressed(Scancode::KpMultiply) {
            self.keys[0xf] = true;
            println!("Key F (KpMultiply) pressed");
        }
        if keyboard_state.is_scancode_pressed(Scancode::Escape) {
            println!("Escape pressed, stopping game");
            *self.running.lock().unwrap() = false;
        }
    }

    pub fn get_keys(&self) -> &[bool; 16] {
        println!("Keys state: {:?}", self.keys);
        &self.keys
    }
}
