use crate::audio::Audio;
use crate::input::InputHandler;
use crate::window::Window;
use rand::Rng;

pub struct CPU {
    pub window: Window,
    pub audio: Audio,
    memory: [u8; 4096],
    v: [u8; 16],
    i: u16,
    pc: u16,
    stack: [u16; 16],
    sp: u8,
    delay_timer: u8,
    sound_timer: u8,
}
// As specified by - https://tobiasvl.github.io/blog/write-a-chip-8-emulator/
// Thank you for the tutorial, sir.
impl CPU {
    pub fn new(window: Window, audio: Audio) -> Self {
        let mut memory = [0u8; 4096];

        // load fontset
        let fontset: [u8; 80] = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80, // F
        ];

        for (i, b) in fontset.iter().enumerate() {
            memory[i] = *b;
        }

        CPU {
            window,
            audio,
            memory,
            v: [0; 16],
            i: 0,
            pc: 0x200,
            stack: [0; 16],
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
        }
    }

    pub fn load_rom(&mut self, rom: &[u8]) -> Result<(), String> {
        if rom.len() > 0x1000 - 0x200 {
            return Err("ROM is too big".to_string());
        }

        for (i, b) in rom.iter().enumerate() {
            self.memory[0x200 + i] = *b;
        }

        Ok(())
    }

    pub fn run_loop(&mut self, input: &InputHandler) -> Result<(), String> {
        let mut executing = true;
        let mut waiting_for_keypress = false;
        let mut store_keypress_in: usize = 0;

        let keys_pressed = input.get_keys();
        for (j, k) in keys_pressed.iter().enumerate() {
            if *k {
                if waiting_for_keypress {
                    executing = true;
                    waiting_for_keypress = false;
                    self.v[store_keypress_in] = j as u8;
                    println!("Stored keypress {} in V{}", j, store_keypress_in);
                    break;
                }
                println!("Key {} pressed!", j);
            }
        }

        if !executing {
            return Ok(());
        }

        // Fetch
        let opcode = ((self.memory[self.pc as usize] as u16) << 8)
            | self.memory[(self.pc + 1) as usize] as u16;
        self.pc += 2;

        // Decode
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;
        let nnn = opcode & 0x0FFF;
        let nn = (opcode & 0x00FF) as u8;
        let n = (opcode & 0x000F) as u8;

        // Execute
        match opcode & 0xF000 {
            0x0000 => match opcode {
                0x00E0 => {
                    self.window.clear_screen();
                    println!("Opcode 00E0: Clear screen");
                }
                0x00EE => {
                    if self.sp > 0 {
                        self.sp -= 1;
                        self.pc = self.stack[self.sp as usize];
                        println!("Opcode 00EE: Return from subroutine to PC={:04X}", self.pc);
                    } else {
                        return Err("Stack underflow".to_string());
                    }
                }
                _ => {
                    println!("Unimplemented machine code routine: {:04X}", opcode);
                }
            },
            0x1000 => {
                self.pc = nnn;
                println!("Opcode 1NNN: Jump to {:04X}", nnn);
            }
            0x2000 => {
                if self.sp < 16 {
                    self.stack[self.sp as usize] = self.pc;
                    self.sp += 1;
                    self.pc = nnn;
                    println!(
                        "Opcode 2NNN: Call subroutine at {:04X}, SP={}",
                        nnn, self.sp
                    );
                } else {
                    return Err("Stack overflow".to_string());
                }
            }
            0x3000 => {
                if self.v[x] == nn {
                    self.pc += 2;
                    println!("Opcode 3XNN: Skip if V{}={} (true)", x, nn);
                } else {
                    println!("Opcode 3XNN: Skip if V{}={} (false)", x, nn);
                }
            }
            0x4000 => {
                if self.v[x] != nn {
                    self.pc += 2;
                    println!("Opcode 4XNN: Skip if V{}!={} (true)", x, nn);
                } else {
                    println!("Opcode 4XNN: Skip if V{}!={} (false)", x, nn);
                }
            }
            0x5000 => {
                if self.v[x] == self.v[y] {
                    self.pc += 2;
                    println!("Opcode 5XY0: Skip if V{}==V{} (true)", x, y);
                } else {
                    println!("Opcode 5XY0: Skip if V{}==V{} (false)", x, y);
                }
            }
            0x6000 => {
                self.v[x] = nn;
                println!("Opcode 6XNN: Set V{}={}", x, nn);
            }
            0x7000 => {
                self.v[x] = self.v[x].wrapping_add(nn);
                println!("Opcode 7XNN: Add {} to V{}, result={}", nn, x, self.v[x]);
            }
            0x8000 => match opcode & 0x000F {
                0x0000 => {
                    self.v[x] = self.v[y];
                    println!("Opcode 8XY0: Set V{}=V{}", x, y);
                }
                0x0001 => {
                    self.v[x] |= self.v[y];
                    println!("Opcode 8XY1: V{} |= V{}", x, y);
                }
                0x0002 => {
                    self.v[x] &= self.v[y];
                    println!("Opcode 8XY2: V{} &= V{}", x, y);
                }
                0x0003 => {
                    self.v[x] ^= self.v[y];
                    println!("Opcode 8XY3: V{} ^= V{}", x, y);
                }
                0x0004 => {
                    let sum = self.v[x] as u16 + self.v[y] as u16;
                    self.v[0xF] = if sum > 255 { 1 } else { 0 };
                    self.v[x] = sum as u8;
                    println!("Opcode 8XY4: V{} += V{}, VF={}", x, y, self.v[0xF]);
                }
                0x0005 => {
                    self.v[0xF] = if self.v[x] >= self.v[y] { 1 } else { 0 };
                    self.v[x] = self.v[x].wrapping_sub(self.v[y]);
                    println!("Opcode 8XY5: V{} -= V{}, VF={}", x, y, self.v[0xF]);
                }
                0x0006 => {
                    self.v[0xF] = self.v[x] & 0x1;
                    self.v[x] >>= 1;
                    println!("Opcode 8XY6: V{} >>= 1, VF={}", x, self.v[0xF]);
                }
                0x0007 => {
                    self.v[0xF] = if self.v[y] >= self.v[x] { 1 } else { 0 };
                    self.v[x] = self.v[y].wrapping_sub(self.v[x]);
                    println!("Opcode 8XY7: V{} = V{}-V{}, VF={}", x, y, x, self.v[0xF]);
                }
                0x000E => {
                    self.v[0xF] = (self.v[x] & 0x80) >> 7;
                    self.v[x] <<= 1;
                    println!("Opcode 8XYE: V{} <<= 1, VF={}", x, self.v[0xF]);
                }
                _ => {
                    return Err(format!("Unknown opcode: {:04X}", opcode));
                }
            },
            0x9000 => {
                if self.v[x] != self.v[y] {
                    self.pc += 2;
                    println!("Opcode 9XY0: Skip if V{}!=V{} (true)", x, y);
                } else {
                    println!("Opcode 9XY0: Skip if V{}!=V{} (false)", x, y);
                }
            }
            0xA000 => {
                self.i = nnn;
                println!("Opcode ANNN: Set I={:04X}", nnn);
            }
            0xB000 => {
                self.pc = nnn + self.v[0] as u16;
                println!("Opcode BNNN: Jump to {:04X}+V0", nnn);
            }
            0xC000 => {
                let random: u8 = rand::thread_rng().r#gen();
                self.v[x] = random & nn;
                println!("Opcode CXNN: V{} = random & {}", x, nn);
            }
            0xD000 => {
                let mut sprite = Vec::new();
                for j in 0..n {
                    sprite.push(self.memory[(self.i + j as u16) as usize]);
                }
                self.v[0xF] = self.window.draw(&sprite, self.v[x], self.v[y]);
                println!(
                    "Opcode DXYN: Draw sprite at ({}, {}), height={}",
                    self.v[x], self.v[y], n
                );
            }
            0xE000 => match opcode & 0x00FF {
                0x009E => {
                    if keys_pressed[self.v[x] as usize] {
                        self.pc += 2;
                        println!("Opcode EX9E: Skip if key V{} pressed (true)", x);
                    } else {
                        println!("Opcode EX9E: Skip if key V{} pressed (false)", x);
                    }
                }
                0x00A1 => {
                    if !keys_pressed[self.v[x] as usize] {
                        self.pc += 2;
                        println!("Opcode EXA1: Skip if key V{} not pressed (true)", x);
                    } else {
                        println!("Opcode EXA1: Skip if key V{} not pressed (false)", x);
                    }
                }
                _ => {
                    return Err(format!("Unknown opcode: {:04X}", opcode));
                }
            },
            0xF000 => match opcode & 0x00FF {
                0x0007 => {
                    self.v[x] = self.delay_timer;
                    println!("Opcode FX07: V{} = delay_timer ({})", x, self.delay_timer);
                }
                0x000A => {
                    executing = false;
                    waiting_for_keypress = true;
                    store_keypress_in = x;
                    println!("Opcode FX0A: Wait for keypress, store in V{}", x);
                }
                0x0015 => {
                    self.delay_timer = self.v[x];
                    println!("Opcode FX15: Set delay_timer=V{}", x);
                }
                0x0018 => {
                    self.sound_timer = self.v[x];
                    println!("Opcode FX18: Set sound_timer=V{}", x);
                }
                0x001E => {
                    self.i += self.v[x] as u16;
                    println!("Opcode FX1E: I += V{}", x);
                }
                0x0029 => {
                    self.i = (self.v[x] * 5) as u16;
                    println!("Opcode FX29: Set I to sprite address for V{}", x);
                }
                0x0033 => {
                    self.memory[self.i as usize] = self.v[x] / 100;
                    self.memory[(self.i + 1) as usize] = (self.v[x] % 100) / 10;
                    self.memory[(self.i + 2) as usize] = self.v[x] % 10;
                    println!("Opcode FX33: Store BCD of V{} at I", x);
                }
                0x0055 => {
                    for j in 0..=x {
                        self.memory[(self.i + j as u16) as usize] = self.v[j];
                    }
                    println!("Opcode FX55: Store V0-V{} at I", x);
                }
                0x0065 => {
                    for j in 0..=x {
                        self.v[j] = self.memory[(self.i + j as u16) as usize];
                    }
                    println!("Opcode FX65: Load V0-V{} from I", x);
                }
                _ => {
                    return Err(format!("Unknown opcode: {:04X}", opcode));
                }
            },
            _ => {
                return Err(format!("Unknown opcode: {:04X}", opcode));
            }
        }

        if self.window.is_open() {
            self.window.refresh()?;
        }

        if self.delay_timer > 0 {
            self.delay_timer -= 1;
            println!("Delay timer decremented to {}", self.delay_timer);
        }
        if self.sound_timer > 0 {
            self.audio.play();
            self.sound_timer -= 1;
            println!(
                "Sound timer decremented to {}, playing audio",
                self.sound_timer
            );
        } else {
            self.audio.pause();
        }

        Ok(())
    }
}
