extern crate tetris_core;
use piston_window::types::Color;
use piston_window::{rectangle, Context, G2d};
use tetris_core::Block;

const BLOCK_SIZE: f64 = 25.0;

pub fn to_coord(game_coord: i32) -> f64 {
    return (game_coord as f64) * BLOCK_SIZE;
}

pub fn draw_block(block: Block, ctx: &Context, g: &mut G2d) {
    let gui_x = to_coord(block.position().x);
    let gui_y = to_coord(block.position().y);
    let color: Color = [
        block.color.red,
        block.color.green,
        block.color.blue,
        block.color.alpha,
    ];
    rectangle(
        color,
        [
            gui_x,
            gui_y,
            BLOCK_SIZE * (block.size().width as f64),
            BLOCK_SIZE * (block.size().height as f64),
        ],
        ctx.transform,
        g,
    );
}
