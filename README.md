Euclidae's CHIP-8 Emulator
A modern CHIP-8 emulator written in Rust, featuring a GTK4-based user interface and SDL3 for graphics and input handling. Developed by Euclidae as a portfolio project to showcase Rust programming skills, this emulator brings classic CHIP-8 games to life with a sleek UI, customizable settings, and robust performance. I aimed to write clean Rust code, avoiding unwrap() where possible, and leveraged SDL3 due to my familiarity with its API. Please post issues and feedback on the GitHub repository to help me improve this project!
Features

GTK4 User Interface: Clean and intuitive interface for selecting ROMs, adjusting settings, and managing recent files.
SDL3 Graphics: Smooth rendering of CHIP-8's 64x32 display with configurable scaling (8x, 10x, 12x).
Audio Support: Toggleable audio with a 440Hz sine wave for CHIP-8 beeps.
Recent ROMs: Automatically tracks up to 5 recently played ROMs for quick access, using Serde for serialization.
Theme Switching: Toggle between dark and light themes for the GTK4 UI.
Customizable Speed: Configurable CPU cycle speed (~500Hz) for accurate gameplay.
Cross-Platform: Built with Rust, GTK4, and SDL3 for compatibility on Linux and Windows.

Installation
Prerequisites

Rust: Install via rustup (curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh).
GTK4: Required for the UI. See below for platform-specific installation.
SDL3: Install development libraries for graphics and input.
Other Dependencies: pkg-config, libpulse (for audio on Linux), and git.

Linux (Arch Linux, Fedora, etc.)

Install dependencies on Arch Linux:sudo pacman -S rust gtk4 sdl3 pkg-config libpulse

On Fedora:sudo dnf install rust gtk4-devel sdl3-devel pkgconf-pkg-config pulseaudio-libs-devel


Clone the repository:git clone https://github.com/Euclidae/chip-8-emulator.git
cd chip-8-emulator


Build and run:cargo build --release
cargo run --release



Troubleshooting GTK4 on Linux
If you encounter an error like:
error: failed to run custom build command for `gtk4-sys v0.7.3`
Caused by:
  Package gtk4 was not found in the pkg-config search path.

Install the GTK4 development libraries:

Arch Linux: sudo pacman -S gtk4
Fedora: sudo dnf install gtk4-develThen re-run cargo build.

Windows

Install MSYS2 from msys2.org.
Open the MSYS2 UCRT64 terminal and install dependencies:pacman -S mingw-w64-ucrt-x86_64-rust mingw-w64-ucrt-x86_64-gtk4 mingw-w64-ucrt-x86_64-sdl3 mingw-w64-ucrt-x86_64-pkgconf mingw-w64-ucrt-x86_64-libpulse


Clone the repository:git clone https://github.com/Euclidae/chip-8-emulator.git
cd chip-8-emulator


Build and run:cargo build --release
cargo run --release



Troubleshooting GTK4 on Windows
If you see an error about missing gtk4.pc or GTK4 libraries during cargo build, ensure GTK4 is installed via MSYS2:
pacman -S mingw-w64-ucrt-x86_64-gtk4

Set the PKG_CONFIG_PATH if needed:
export PKG_CONFIG_PATH=/ucrt64/lib/pkgconfig:$PKG_CONFIG_PATH

Then re-run cargo build.
Usage

Launch the emulator:cargo run


The GTK4 window ("Euclidae's CHIP-8 Emulator") opens with:
A dropdown for recent ROMs.
A "Select CHIP-8 ROM" button to browse for .ch8 files.
A resolution scale dropdown (8x, 10x, 12x).
An audio toggle checkbox.
A theme toggle button (dark/light).
A "Clear Recent ROMs" button.


Select a ROM (e.g., Pong2.ch8 from the roms/ directory):
The GTK window hides, and an SDL window opens.
Play the game using the key mappings below.
Press Escape to close the SDL window and return to the GTK UI.



Key Mappings
The emulator maps keyboard inputs to CHIP-8's 16-key keypad, suitable for games like Pong2.ch8:



CHIP-8 Key
Keyboard Key
Description (e.g., Pong2)



7
A
Player 1 Up


4
Q
Player 1 Down


B
/
Player 2 Up


F
* (Keypad)
Player 2 Down


0-3, 5-9, A, C-D
X, Kp1-Kp4, W, E, S, D, Z, C, R, F, V
General inputs


Screenshots

GTK4 UI: (Placeholder for UI screenshot showing ROM selection and settings.)
Pong2 Gameplay: (Placeholder for SDL window showing Pong2.ch8 gameplay.)

Credits
This project was built with the help of the following resources, which greatly improved my understanding of the CHIP-8 architecture and SDL3 API:

orcalinux/chip8-emulator: CHIP-8 emulator tutorial.
tobiasvl.github.io: A detailed guide on writing a CHIP-8 emulator.
freecodecamp.org: A comprehensive CHIP-8 emulator tutorial.
Daniel's Udemy Course: A C-based CHIP-8 emulator course, used sparingly for additional insights.
Serde Documentation: Used for JSON serialization of recent ROMs.

Cross-referencing these resources was invaluable for the implementation. Special thanks to their authors for making CHIP-8 emulation accessible!
Contributing
This is a portfolio project, and I’d greatly appreciate feedback to improve my code! Please:

Fork the repository.
Create a feature branch (git checkout -b feature/YourFeature).
Commit changes (git commit -m 'Add YourFeature').
Push to the branch (git push origin feature/YourFeature).
Open a pull request or post issues on GitHub.

License
This project is licensed under the MIT License. See LICENSE for details.
About the Author
Euclidae is a passionate developer with interests in Rust programming, RPG games, and anime (especially Naruto). Check out more projects at github.com/Euclidae.
