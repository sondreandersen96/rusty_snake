use macroquad::prelude::*;

use crate::{SCALE, snake::Snake, FOOD_COLOR, BORDER_THICKNESS, BORDER_COLOR};


pub struct Food {
    pos_x: f32,
    pos_y: f32,
    points: u32,
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

    pub fn get_points(&self) -> u32 {
        return self.points;
    }

    pub fn render(&self) {
        let border_w = (screen_width() / SCALE) + (BORDER_THICKNESS / 2.0);
        let border_h = (screen_width() / SCALE) + (BORDER_THICKNESS / 2.0);

        draw_rectangle(self.pos_x, self.pos_y, screen_width() / SCALE, screen_height() / SCALE, FOOD_COLOR);
        draw_rectangle_lines(self.pos_x, self.pos_y, border_w, border_h, BORDER_THICKNESS, BORDER_COLOR);
    }
}

