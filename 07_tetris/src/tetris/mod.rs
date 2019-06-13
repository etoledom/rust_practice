mod active_figure;
mod board;
mod figure;
mod game;
use active_figure::ActiveFigure;
use board::Board;
use figure::{Figure, FigureType, Matrix};
extern crate utilities;
pub use game::{Game, Randomizer};
use utilities::block::Block;
use utilities::geometry::*;
use utilities::graphics::Color;

mod move_validator;
