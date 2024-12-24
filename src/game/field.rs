use raylib::{color::Color, prelude::{RaylibDraw, RaylibDrawHandle}};

use super::{game, snake::Snake};

pub const FIELD_WIDTH: usize = 20;
pub const FIELD_HEIGHT: usize = 20;
pub const BOX_WHIDTH: i32 = game::WIDTH / FIELD_WIDTH as i32;
pub const BOX_HEIGHT: i32 = game::HEIGHT / FIELD_HEIGHT as i32;


pub fn draw_field(draw_handle: &mut RaylibDrawHandle){
    for h in 0..FIELD_HEIGHT  {
        for w in 0..FIELD_WIDTH {
            if h % 2 == 0 {
                if w % 2 == 0 {
                    draw_handle.draw_rectangle(
                        w as i32 * BOX_WHIDTH, h as i32 * BOX_WHIDTH, BOX_WHIDTH,
                        BOX_HEIGHT, Color::new(12, 12, 12, 255));
                }
                else {
                    draw_handle.draw_rectangle(
                        w as i32 * BOX_WHIDTH, h as i32 * BOX_WHIDTH, BOX_WHIDTH,
                        BOX_HEIGHT, Color::new(8, 8, 8, 255));
                }
            }
            else {
                if w % 2 == 0 {
                    draw_handle.draw_rectangle(
                        w as i32 * BOX_WHIDTH, h as i32 * BOX_WHIDTH, BOX_WHIDTH,
                        BOX_HEIGHT, Color::new(8, 8, 8, 255));
                }
                else {
                    draw_handle.draw_rectangle(
                        w as i32 * BOX_WHIDTH, h as i32 * BOX_WHIDTH, BOX_WHIDTH,
                        BOX_HEIGHT, Color::new(12, 12, 12, 255));
                }
            }
        }
    }
}