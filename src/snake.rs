use macroquad::prelude::*;

use crate::{direction::Direction, SCALE, SNAKE_BODY_COLOR, SNAKE_HEAD_COLOR, BORDER_COLOR, BORDER_THICKNESS};

pub struct Snake {
    pub pos_x: f32,
    pub pos_y: f32,
    length: i32,
    pub direction: Direction,
    pub history: Vec<(f32, f32)>
}

impl Snake {
    pub fn new(start_pos_x: f32, start_pos_y: f32) -> Snake {
        let snake = Snake {
            pos_x: start_pos_x,
            pos_y: start_pos_y,
            length: 0,
            direction: Direction::RIGHT,
            history: vec![],
        };
        snake 
    }

    pub fn set_direction(&mut self, dir: Direction) {
        self.direction = dir;
    }

    pub fn move_snake(&mut self) {
        self.history.push((self.pos_x, self.pos_y));

        let move_length = screen_width() / SCALE;

        match self.direction {
            Direction::UP => self.pos_y -= move_length,
            Direction::DOWN => self.pos_y += move_length,
            Direction::LEFT => self.pos_x -= move_length,
            Direction::RIGHT => self.pos_x += move_length,
        }

        // Making sure snake does not grow more than it should 
        while self.history.len() > self.length.try_into().unwrap() {
            self.history.remove(0);
        }
    }

    // Adds to length of snake
    pub fn eat(&mut self) {
        self.length += 1; 
    }


    pub fn render(&self) {
        let border_w = (screen_width() / SCALE) + (BORDER_THICKNESS / 2.0);
        let border_h = (screen_width() / SCALE) + (BORDER_THICKNESS / 2.0);
        
        draw_rectangle(self.pos_x, self.pos_y, screen_width() / SCALE, screen_height() / SCALE, SNAKE_HEAD_COLOR);
        draw_rectangle_lines(self.pos_x, self.pos_y, border_w, border_h, BORDER_THICKNESS, BORDER_COLOR);
        
        // Draw rest of snake (based on length)
        for h in &self.history {

            draw_rectangle(h.0, h.1, screen_width() / SCALE, screen_height() / SCALE, SNAKE_BODY_COLOR);   
            draw_rectangle_lines(h.0, h.1, border_w, border_w, BORDER_THICKNESS, BORDER_COLOR);
        }
    }
}



