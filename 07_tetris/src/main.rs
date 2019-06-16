mod draw;
extern crate tetris_core;
use draw::{draw_block, to_coord};

use piston_window::types::Color as PistonColor;
use piston_window::*;
use rand::Rng;
use tetris_core::{Game, Randomizer, Size};

const BACK_COLOR: PistonColor = [0.5, 0.5, 0.5, 1.0];

struct Rand;
impl Randomizer for Rand {
    fn random_between(&self, lower: i32, higher: i32) -> i32 {
        let mut rng = rand::thread_rng();
        return rng.gen_range(lower, higher);
    }
}

fn main() {
    let game_size = Size {
        height: 20,
        width: 10,
    };

    let mut window: PistonWindow = WindowSettings::new(
        "Tetris",
        [
            to_coord(game_size.width as i32),
            to_coord(game_size.height as i32),
        ],
    )
    .exit_on_esc(true)
    .build()
    .unwrap();
    let rand = Rand {};
    let mut game = Game::new(&game_size, Box::new(rand));
    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            if game.is_game_over() {
                game = Game::new(&game_size, Box::new(Rand {}));
            }
            match key {
                Key::Left => game.move_left(),
                Key::Right => game.move_right(),
                Key::Space => game.rotate(),
                Key::Down => game.move_down(),
                _ => continue,
            }
        }
        window.draw_2d(&event, |ctx, g2d| {
            clear(BACK_COLOR, g2d);
            let game_blocks = game.draw();
            for block in game_blocks {
                draw_block(block, &ctx, g2d);
            }
        });

        event.update(|arg| game.update(arg.dt));
    }
}
