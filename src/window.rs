// Stole some of my own code from my SDL3 raycaster and Brick Breaker then modified it for Chip-8. Hopefully, I didn't forget to change anything.
// I literally had brick breaker as the title of my raycaster :|

use sdl3::pixels::Color;
use sdl3::rect::Point;
use sdl3::render::Canvas;
use sdl3::video::Window as SDLWindow;

use crate::util::is_bit_set;

const WIDTH: usize = 64;
const HEIGHT: usize = 32;
const PX_OFF: u32 = 0x81c784;
const PX_ON: u32 = 0x29302a;

pub struct Window {
    canvas: Canvas<SDLWindow>,
    framebuffer: [u32; WIDTH * HEIGHT],
}

impl Window {
    pub fn new(title: &str, scale: u32) -> Result<Window, String> {
        let sdl = sdl3::init().map_err(|e| e.to_string())?;
        let video = sdl.video().map_err(|e| e.to_string())?;

        let window = video
            .window(
                title,
                (WIDTH as u32 * scale) as u32,
                (HEIGHT as u32 * scale) as u32,
            )
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;

        let mut canvas = window.into_canvas();

        canvas.set_scale(scale as f32, scale as f32).unwrap();
        canvas.set_draw_color(Color::RGB(
            (PX_OFF >> 16) as u8,
            (PX_OFF >> 8) as u8,
            PX_OFF as u8,
        ));
        canvas.clear();
        canvas.present();

        Ok(Window {
            canvas,
            framebuffer: [PX_OFF; WIDTH * HEIGHT],
        })
    }

    pub fn clear_screen(&mut self) {
        for j in 0..self.framebuffer.len() {
            self.framebuffer[j] = PX_OFF;
        }
    }

    pub fn draw(&mut self, bytes: &Vec<u8>, init_x: u8, init_y: u8) -> u8 {
        let mut collision: u8 = 0;
        for (k, b) in bytes.iter().enumerate() {
            for j in 0..8 {
                let x = (init_x as usize + j) % WIDTH;
                let y = (init_y as usize + k) % HEIGHT;
                let coord = (y * WIDTH) + x;
                let is_old_set = self.framebuffer[coord] == PX_ON;
                self.framebuffer[coord] = if is_bit_set(b, (8 - j - 1) as u8) {
                    if is_old_set {
                        collision = 1;
                        PX_OFF
                    } else {
                        PX_ON
                    }
                } else {
                    self.framebuffer[coord]
                };
            }
        }
        collision
    }

    pub fn refresh(&mut self) -> Result<(), String> {
        self.canvas.set_draw_color(Color::RGB(
            (PX_OFF >> 16) as u8,
            (PX_OFF >> 8) as u8,
            PX_OFF as u8,
        ));
        self.canvas.clear();

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let idx = y * WIDTH + x;
                let pixel = self.framebuffer[idx];
                self.canvas.set_draw_color(Color::RGB(
                    (pixel >> 16) as u8,
                    (pixel >> 8) as u8,
                    pixel as u8,
                ));
                self.canvas
                    .draw_point(Point::new(x as i32, y as i32))
                    .map_err(|e| e.to_string())?;
            }
        }

        self.canvas.present();
        Ok(())
    }

    pub fn is_open(&self) -> bool {
        true
    }
}
