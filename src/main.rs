#![allow(unused_variables)]

use std::{time::{Instant, Duration}};
use macroquad::prelude::*;

mod controller;
mod snake;
mod direction;
mod food;

use macroquad::prelude::{screen_height, screen_width};
use snake::Snake;
use controller::Controller;

const SCALE: f32 = 20.0; 
const SNAKE_BODY_COLOR: macroquad::color::Color = color_u8!(255, 255, 0, 255);
const SNAKE_HEAD_COLOR: macroquad::color::Color = color_u8!(255, 0, 0, 255);
const FOOD_COLOR: macroquad::color::Color = color_u8!(0, 255, 0, 255);
const BORDER_COLOR: macroquad::color::Color = color_u8!(20, 20, 20, 255);
const BORDER_THICKNESS: f32 = 4.0; // Should be an even number, since it is often divided by two and used for pixels 
const START_UPDATE_INTERVAL: Duration = Duration::from_millis(1000);

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
    let mut snake = Snake::new(screen_width() / 2.0, screen_height() / 2.0);
    let mut update_interval = Duration::from_millis(1000);

    let mut controller = Controller::new(snake, START_UPDATE_INTERVAL);

    loop {
        clear_background(GRAY);

        if !controller.game_over {
            game(&mut controller); 
            draw_text(format!("SCORE: {}", controller.points).as_str(), 10.0, 44.0, 22.0, DARKGRAY);
        } else {
            game_over(&mut controller);
        }
        
        // Render debug information 
        draw_text(format!("FRAME: {}, DIRECTION: {}, FPS: {}", controller.frame_counter, controller.snake.direction.as_str(), get_fps()).as_str(), 10.0, 20.0, 22.0, DARKGRAY);

        controller.frame_counter += 1;
        next_frame().await
    }
}

fn game(controller: &mut Controller) {
    // Move snake every x seconds or on userinput 
    let duration = Instant::now() - controller.prev_update_time;
    if controller.handle_arrow_keys() || duration > controller.update_interval {

        controller.snake.move_snake();
        controller.prev_update_time = Instant::now();
        
        controller.spawn_food_at_random_time();
        
        println!("Snake history [{}]: {:?}", controller.frame_counter, controller.snake.history);
    
        // Check for food intersection 
        controller.test_for_intersection();

        // Test for wall collision 
        if controller.test_for_wall_collision() || controller.test_for_tail_bite(){
            println!("--------- GAME OVER ---------"); 
            controller.game_over = true; 
        }
        
    }
    
    // Render food 
    for food in &mut controller.food {
        food.render();
    }

    // Render snake 
    controller.snake.render(); 
}

fn game_over(controller: &mut Controller) {
    draw_text(format!("GAME OVER. Score was: {}", controller.points).as_str(), 150.0, screen_height() / 2.0, 30.0, color_u8!(0, 0, 0, 255));
}

