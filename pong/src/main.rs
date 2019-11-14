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
    left_vel: i32,
    left_pos: i32,
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
        const FOREGROUND: [f32; 4] = [0.0, 0.5, 1.0, 1.0];

        let left = rectangle::square(0.0, 0.0, 50.0);
        let left_pos = self.left_pos as f64;
    }
}

fn main() {
    println!("Hello, world!");
}

