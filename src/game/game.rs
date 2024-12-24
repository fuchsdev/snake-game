use raylib::{RaylibHandle, RaylibThread};
use raylib::{color::Color, prelude::RaylibDraw, ffi::KeyboardKey::*};

use crate::game::game_state::GameState;
use crate::game::snake::Snake;

use super::field::{FIELD_HEIGHT, FIELD_WIDTH};
use super::item_handler::ItemHandler;
use super::{field, item_handler, snake};
use super::snake::Direction;

pub const WIDTH: i32 = 800;
pub const HEIGHT: i32 = 800;
const SPEED: f64 = 0.2;


pub fn run(){
    let (mut rl, thread) = raylib::init()
    .size(WIDTH, HEIGHT)
    .title("Snake Game")
    .vsync()
    .build();

    let mut game_state = GameState::new();
    let mut snake = Snake::new();
    let mut item_handler = ItemHandler::new(&snake);

    let mut started = false;
    let mut game_over = false;

    let mut last_time = rl.get_time();

    while !rl.window_should_close() {

        let current_time = rl.get_time();

        if !started && rl.is_key_pressed(KEY_SPACE) {
            started = true;
        }
        
        //UPDATE//
        if started && !game_over{
            if rl.is_key_pressed(KEY_UP) || rl.is_key_pressed(KEY_W) 
            && snake.direction != Direction::Up && snake.direction != Direction::Down {
                snake.direction = Direction::Up;
                snake.move_snake();
                last_time = current_time;
            }
            if rl.is_key_pressed(KEY_DOWN) || rl.is_key_pressed(KEY_S) 
                && snake.direction != Direction::Down && snake.direction != Direction::Up {
                snake.direction = Direction::Down;
                snake.move_snake();
                last_time = current_time;
            }
            if rl.is_key_pressed(KEY_LEFT) || rl.is_key_pressed(KEY_A) 
                && snake.direction != Direction::Left && snake.direction != Direction::Right {
                snake.direction = Direction::Left;
                snake.move_snake();
                last_time = current_time;
            }
            if rl.is_key_pressed(KEY_RIGHT) || rl.is_key_pressed(KEY_D)
                && snake.direction != Direction::Right && snake.direction != Direction::Left {
                snake.direction = Direction::Right;
                snake.move_snake();
                last_time = current_time;
            }

            if current_time - last_time >= SPEED && started {
                last_time = current_time;
                snake.move_snake();
            }

            if snake.check_game_over(field::FIELD_WIDTH as i32, field::FIELD_HEIGHT as i32) {
                game_over = true;
            }

            item_handler.check_for_item(&mut snake);
        }
        else if game_over {
            if rl.is_key_pressed(KEY_SPACE) {
                game_over = false;
                snake = Snake::new();
                item_handler = ItemHandler::new(&snake);
            }
        }

        //DRAW//
        draw(&mut rl, &thread, &mut snake, &mut item_handler, started, game_over);

        //game_state.update(&mut draw_handle);
    }
}

fn draw(rl: &mut RaylibHandle, thread: &RaylibThread, snake: &mut Snake, 
        item_handler: &mut ItemHandler, started: bool, game_over: bool){
        let mut draw_handle = rl.begin_drawing(&thread);
        draw_handle.clear_background(Color::WHITE);
        field::draw_field(&mut draw_handle);
        snake.draw_snake(&mut draw_handle);
        item_handler.draw_items(snake, &mut draw_handle);
        draw_handle.draw_text(&format!("Score: {}", snake.boxes.len() - 3), 5, 5, 25, Color::WHITE);

        if !started {
            draw_handle.draw_text("Press 'SPACE' to play...", 115, 250, 50, Color::WHITE);
        } 
        else if game_over {
            draw_handle.draw_text("GAME OVER!", 240, 300, 50, Color::WHITE);
            draw_handle.draw_text("Press 'SPACE' to play...", 115, 400, 50, Color::WHITE);
        }
}