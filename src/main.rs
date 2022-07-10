#![allow(unused_variables)]

use std::{time::{Instant, Duration}};
use macroquad::prelude::*;

mod controller;
mod snake;
mod direction;
mod food;

use snake::Snake;
use controller::Controller;

const SCALE: f32 = 20.0; 
const SNAKE_BODY_COLOR: macroquad::color::Color = color_u8!(255, 255, 0, 255);
const SNAKE_HEAD_COLOR: macroquad::color::Color = color_u8!(255, 0, 0, 255);
const FOOD_COLOR: macroquad::color::Color = color_u8!(0, 255, 0, 255);
const BORDER_COLOR: macroquad::color::Color = color_u8!(20, 20, 20, 255);
const BORDER_THICKNESS: f32 = 4.0; // Should be an even number, since it is often divided by two and used for pixels 

// Window setup
fn window_conf() -> Conf {
    Conf {
        window_height: 600,
        window_width: 600,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut duration: Duration;
    let mut snake = Snake::new(screen_width() / 2.0, screen_height() / 2.0);
    let mut update_interval = Duration::from_millis(1000);

    let mut controller = Controller::new(snake);

    loop {
        clear_background(GRAY);

        // Move snake every x seconds or on userinput 
        duration = Instant::now() - controller.prev_update_time;
        if controller.handle_arrow_keys() || duration > update_interval {

            controller.snake.move_snake();
            controller.prev_update_time = Instant::now();
            
            controller.spawn_food_at_random_time();
            
            println!("Snake history [{}]: {:?}", controller.frame_counter, controller.snake.history);
        
            // Check for food intersection 
            controller.test_for_intersection();
        }
        
        // Render food 
        for food in &mut controller.food {
            food.render();
        }

        // Render snake 
        controller.snake.render(); 
        
        // Render debug information 
        draw_text(format!("FRAME: {}, DIRECTION: {}, FPS: {}", controller.frame_counter, controller.snake.direction.as_str(), get_fps()).as_str(), 10.0, 20.0, 22.0, DARKGRAY);

        controller.frame_counter += 1;
        next_frame().await
    }
}


struct Model {}
struct View {}



