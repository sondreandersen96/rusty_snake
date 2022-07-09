use macroquad::prelude::*;

use crate::{SCALE, snake::Snake};


pub struct Food {
    pos_x: f32,
    pos_y: f32,
    points: u8,
}

impl Food {
    pub fn new(start_pos_x: f32, start_pos_y: f32) -> Food {
        Food {
            pos_x: start_pos_x,
            pos_y: start_pos_y,
            points: 1, 
        }
    }

    pub fn intersected(&self, snake: &Snake) -> bool {
        if (self.pos_x == snake.pos_x) && (self.pos_y == snake.pos_y) {
            println!("INTERSECTED !!!");
            return true; 
        }
        false 
    }

    pub fn render(&self) {
        draw_rectangle(self.pos_x, self.pos_y, screen_width() / SCALE, screen_height() / SCALE, GREEN);
    }
}

