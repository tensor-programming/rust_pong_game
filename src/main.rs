extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use std::process;
use piston::window::WindowSettings;
use piston::event_loop::{EventSettings, Events};
use piston::input::{Button, Key, PressEvent, ReleaseEvent, RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};

pub struct App {
    gl: GlGraphics,
    left_score: i32,
    left_pos: i32,
    left_vel: i32,
    right_score: i32,
    right_pos: i32,
    right_vel: i32,
    ball_x: i32,
    ball_y: i32,
    vel_x: i32,
    vel_y: i32,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BACKGROUND: [f32; 4] = [0.0, 0.5, 0.5, 1.0];
        const FOREGROUND: [f32; 4] = [0.0, 0.0, 1.0, 1.0];

        let left = rectangle::square(0.0, 0.0, 50.0);
        let left_pos = self.left_pos as f64;
        let right = rectangle::square(0.0, 0.0, 50.0);
        let right_pos = self.right_pos as f64;

        let ball = rectangle::square(0.0, 0.0, 10.0);
        let ball_x = self.ball_x as f64;
        let ball_y = self.ball_y as f64;

        self.gl.draw(args.viewport(), |c, gl| {
            clear(BACKGROUND, gl);
            rectangle(FOREGROUND, left, c.transform.trans(-40.0, left_pos), gl);
            rectangle(
                FOREGROUND,
                right,
                c.transform.trans(args.window_size[0] as f64 - 10.0, right_pos),
                gl,
            );
            rectangle(FOREGROUND, ball, c.transform.trans(ball_x, ball_y), gl);
        });
    }

    fn update(&mut self, _args: &UpdateArgs) {
        if (self.left_vel == 1 && self.left_pos < 291)
            || (self.left_vel == -1 && self.left_pos >= 1)
        {
            self.left_pos += self.left_vel;
        }
        if (self.right_vel == 1 && self.right_pos < 291)
            || (self.right_vel == -1 && self.right_pos >= 1)
        {
            self.right_pos += self.right_vel;
        }
        self.ball_x += self.vel_x;
        if self.ball_x > 502 {
            self.vel_x = -self.vel_x;
            if self.ball_y < self.right_pos || self.ball_y > self.right_pos + 50 {
                self.left_score += 1;
                if self.left_score >= 5 {
                    println!("Left wins!");
                    process::exit(0);
                }
                self.ball_x = 256;
                self.ball_y = 171;
            }
        }
        if self.ball_x < 1 {
            self.vel_x = -self.vel_x;
            if self.ball_y < self.left_pos || self.ball_y > self.left_pos + 50 {
                self.right_score += 1;
                if self.right_score >= 5 {
                    println!("Right wins!");
                    process::exit(0);
                }
                self.ball_x = 256;
                self.ball_y = 171;
            }
        }

        self.ball_y += self.vel_y;
        if self.ball_y > 332 || self.ball_y < 1 {
            self.vel_y = -self.vel_y;
        }
    }

    fn press(&mut self, args: &Button) {
        if let &Button::Keyboard(key) = args {
            match key {
                Key::Up => {
                    self.right_vel = -1;
                }
                Key::Down => {
                    self.right_vel = 1;
                }
                Key::W => {
                    self.left_vel = -1;
                }
                Key::S => {
                    self.left_vel = 1;
                }
                _ => {}
            }
        }
    }

    fn release(&mut self, args: &Button) {
        if let &Button::Keyboard(key) = args {
            match key {
                Key::Up => {
                    self.right_vel = 0;
                }
                Key::Down => {
                    self.right_vel = 0;
                }
                Key::W => {
                    self.left_vel = 0;
                }
                Key::S => {
                    self.left_vel = 0;
                }
                _ => {}
            }
        }
    }
}

fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: GlutinWindow = WindowSettings::new("Pong", [512, 342])
        /* .opengl(opengl) */
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App {
        gl: GlGraphics::new(opengl),
        left_score: 0,
        left_pos: 1,
        left_vel: 0,
        right_score: 0,
        right_pos: 1,
        right_vel: 0,
        ball_x: 0,
        ball_y: 0,
        vel_x: 1,
        vel_y: 1,
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }

        if let Some(b) = e.press_args() {
            app.press(&b);
        }

        if let Some(b) = e.release_args() {
            app.release(&b);
        }
    }
}