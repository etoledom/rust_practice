mod draw;
mod snake;
use draw::{draw_block, to_coord};
use rand::Rng;

use piston_window::types::Color as PistonColor;
use piston_window::*;
use snake::geometry::Size;
use snake::{Button as GameButton, Game, Randomizer};

const BACK_COLOR: PistonColor = [0.5, 0.5, 0.5, 1.0];

struct Rand;
impl Randomizer for Rand {
    fn random_between(&self, lower: u32, higher: u32) -> u32 {
        let mut rng = rand::thread_rng();
        return rng.gen_range(lower, higher);
    }
}

fn button_from_key(key: Key) -> Option<GameButton> {
    match key {
        Key::Up => Some(GameButton::Up),
        Key::Right => Some(GameButton::Right),
        Key::Down => Some(GameButton::Down),
        Key::Left => Some(GameButton::Left),
        _ => None,
    }
}

fn main() {
    let game_size = Size {
        height: 20,
        width: 20,
    };
    let randomizer = Rand {};
    let mut window: PistonWindow = WindowSettings::new(
        "Snake",
        [to_coord(game_size.width), to_coord(game_size.height)],
    )
    .exit_on_esc(true)
    .build()
    .unwrap();

    let mut game = Game::new(game_size);
    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            if let Some(button) = button_from_key(key) {
                game.button_pressed(button);
            }
        }
        window.draw_2d(&event, |ctx, g2d| {
            clear(BACK_COLOR, g2d);
            let game_blocks = game.draw();
            for block in game_blocks {
                draw_block(block, &ctx, g2d);
            }
        });

        event.update(|arg| game.update(arg.dt, &randomizer));
    }
}
