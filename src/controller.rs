use ::rand::Rng;

use macroquad::prelude::*;
use std::{time::{Instant, Duration}};


use crate::{food::Food, snake::Snake, direction::Direction, SCALE};

pub struct Controller {
    pub snake: Snake,
    pub prev_update_time: Instant,
    pub frame_counter: i64,
    pub food: Vec<Food>,
    pub update_interval: Duration,
    pub game_over: bool,
    pub points: u32, 
}
impl Controller {
    pub fn new(snake: Snake, update_interval: Duration) -> Controller {
        Controller {
            snake, 
            prev_update_time: Instant::now(),
            frame_counter: 0,
            food: vec![],
            update_interval,
            game_over: false, 
            points: 0,  
        }
    }

    pub fn handle_arrow_keys(&mut self) -> bool {
        let mut some_key_was_pressed = false;
        let mut direction: Direction = Direction::LEFT; // Does not matter which direction is set here, but some must be set, otherwise compiler will complain
        if is_key_pressed(KeyCode::Up) 
            && self.snake.direction != Direction::UP 
            && self.snake.direction != Direction:: DOWN {
            some_key_was_pressed = true;
            direction = Direction::UP;        
        }
        if is_key_pressed(KeyCode::Down) 
            && self.snake.direction != Direction::DOWN 
            && self.snake.direction != Direction::UP {
            some_key_was_pressed = true;
            direction = Direction::DOWN;
        }
        if is_key_pressed(KeyCode::Left) 
            && self.snake.direction != Direction::LEFT 
            && self.snake.direction != Direction::RIGHT {
            some_key_was_pressed = true;
            direction = Direction::LEFT;
        }
        if is_key_pressed(KeyCode::Right) 
            && self.snake.direction != Direction::RIGHT
            && self.snake.direction != Direction::LEFT {
            some_key_was_pressed = true;
            direction = Direction::RIGHT;
        }

        if some_key_was_pressed {
            self.handle_snake_direction_change(direction);
            return true; 
        }
        false 
    }
    
    fn handle_snake_direction_change(&mut self, new_direction: Direction) {
        if new_direction != self.snake.direction {
            self.snake.set_direction(new_direction);
            //self.snake.move_snake();    
            self.reset_last_update_timer();
        }
    }
    
    fn reset_last_update_timer(&mut self) {
        self.prev_update_time = Instant::now();
    }

    pub fn spawn_food_at_random_time(&mut self) {
            
        let mut rng: f32 = 0.0;
    
        let spawn_prob = 50;
        let random_number: i32 = ::rand::thread_rng().gen_range(0, 100);
        
        
        if random_number < spawn_prob {
            println!("Spawining new food");
            
            let spawn_position = Controller::get_random_coordinates();
            let x = spawn_position.0 as f32;
            let y = spawn_position.1 as f32;

            let f = Food::new(x, y);
            self.food.push(f);
        }
    }

    pub fn test_for_intersection(&mut self) {
        let food_to_be_removed: Vec<Food> = vec![];

        // Check if some food is eaten and remove such foods 
        // .. from foods vector (using retain)
        self.food.retain( | f | {
            if f.intersected(&self.snake)  {
                self.snake.eat();
                self.points += f.get_points(); 
                return false;
            }
            true
        });
    }

    // Returns true if wall collision has occured
    pub fn test_for_wall_collision(&mut self) -> bool {
        let snake_pos: (f32, f32) = (self.snake.pos_x, self.snake.pos_y);
        let snake_size = screen_width() / SCALE; 

        if snake_pos.0 < 0.0 {
            println!("WALL COLLISION LEFT!");
            return true;
        }
        if snake_pos.0 + snake_size > screen_width() {
            println!("WALL COLLISION RIGHT!");
            return true;
        }
        if snake_pos.1 < 0.0 {
            println!("WALL COLLISION TOP!");
            return true;
        }
        if snake_pos.1 + snake_size > screen_height() {
            println!("WALL COLLISION BOTTOM!");
            return true;
        }
        false 
    }

    // Returns true if tail was bitten
    pub fn test_for_tail_bite(&mut self) -> bool {
        let snake = &self.snake;
        let b = &self.snake.history; 

        for item in b {
            if snake.pos_x == item.0 && snake.pos_y == item.1 {
                return true; 
            }
        }
        false
    }
    
    fn get_random_coordinates() -> (f32, f32){
        /*
        - There are SCALE number of colmums/rows (assuming square grid)
        - Each cell on the board is screen_width (or screen_height) / SCALE
        */

        let size = screen_width() / SCALE;
        
        let random_x = ::rand::thread_rng().gen_range(0, SCALE as i32) as f32;
        let random_y = ::rand::thread_rng().gen_range(0, SCALE as i32) as f32;
    
        let start_pos_x = random_x * size;
        let start_pos_y = random_y * size;

        (start_pos_x, start_pos_y)
    }

}