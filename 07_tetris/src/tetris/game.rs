extern crate utilities;
use super::active_figure::ActiveFigure;
use super::figure::{Figure, FigureType};
use super::Board;
pub use utilities::block::Block;
use utilities::geometry::{Point, Size};
use utilities::graphics::Color;

const MOVING_PERIOD: f64 = 0.2; //secs
const BACKGROUND_COLOR: Color = Color {
    red: 1.0,
    green: 1.0,
    blue: 1.0,
    alpha: 1.0,
};

struct Game {
    board: Board,
    points: u128,
    active: ActiveFigure,
    waiting_time: f64,
}

impl Game {
    fn new(size: Size) -> Game {
        let active = ActiveFigure::new(FigureType::T, Point { x: 0, y: 0 });
        let board = Board::new(size);
        return Game {
            board,
            points: 0,
            active,
            waiting_time: 0.0,
        };
    }

    pub fn draw(&self) -> Vec<Block> {
        let board = self.draw_board();
        let figure = self.draw_active_figure();
        return board.iter().chain(&figure).cloned().collect();
    }

    fn draw_active_figure(&self) -> Vec<Block> {
        let figure = self.active.to_cartesian();
        return figure
            .iter()
            .map(|point| Block::new(point.x, point.y, 1, 1, self.active.color()))
            .collect();
    }

    fn draw_board(&self) -> Vec<Block> {
        let mut blocks = vec![];
        for y in 0..self.board.height() {
            for x in 0..self.board.width() {
                if let Some(square) = self.board.figure_at_xy(x, y) {
                    let block = Block::new(x, y, 1, 1, square.color());
                    blocks.push(block);
                }
            }
        }
        return blocks;
    }

    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;
        if self.waiting_time > MOVING_PERIOD {
            self.update_game();
            self.waiting_time = 0.0;
        }
    }

    fn update_game(&mut self) {
        if self.can_move_down() {
            self.move_active_figure_down();
        }
    }

    fn can_move_down(&self) -> bool {
        return !self.is_at_the_bottom() && !self.will_colide_with_block();
    }

    fn is_at_the_bottom(&self) -> bool {
        return self.active.to_cartesian().iter().fold(false, |acc, point| {
            acc || point.y == ((self.board.height() - 1) as u32)
        });
    }

    fn will_colide_with_block(&self) -> bool {
        let moved_down_points = self.active_figure_moved_down().to_cartesian();
        for point in moved_down_points {
            if self.board_contains(point) {
                return true;
            }
        }
        return false;
    }

    fn board_contains(&self, point: Point) -> bool {
        return self.board.figure_at_xy(point.x, point.y).is_some();
    }

    fn active_figure_moved_down(&self) -> ActiveFigure {
        return self.active.updating_position_by_xy(0, 1);
    }

    fn move_active_figure_down(&mut self) {
        let new_active = self.active_figure_moved_down();
        self.update_active_with(new_active);
    }

    fn rotate_active_figure(&mut self) {
        let new_active = self.active.rotated();
        self.update_active_with(new_active);
    }

    fn update_active_with(&mut self, new_active: ActiveFigure) {
        self.active = new_active;
    }
}

#[cfg(test)]
mod game_tests {
    use super::*;
    #[test]
    fn test_active_figure_is_added_to_the_board() {
        let game = get_game();
        let active_points = game.active.to_cartesian();
        let drawed_points = draw_to_cartesian(game.draw());

        assert_eq!(drawed_points, active_points);
    }
    #[test]
    fn test_active_figure_moves_down() {
        let mut game = get_game();
        let first_position = game.active.to_cartesian();
        let expected: Vec<Point> = first_position
            .iter()
            .map(|point| Point {
                x: point.x,
                y: point.y + 1,
            })
            .collect();

        game.move_active_figure_down();
        let drawed_points = draw_to_cartesian(game.draw());

        assert_eq!(drawed_points, expected);
    }
    #[test]
    fn test_rotate_active_figure() {
        let mut game = get_game();
        let rotated = game.active.rotated();
        game.rotate_active_figure();
        let drawed_points = draw_to_cartesian(game.draw());
        assert_eq!(drawed_points, rotated.to_cartesian());
    }
    #[test]
    fn test_active_figure_is_at_the_bottom() {
        let mut game = Game::new(Size {
            height: 4,
            width: 20,
        });
        game.move_active_figure_down();
        assert!(!game.is_at_the_bottom());
        game.move_active_figure_down();
        assert!(game.is_at_the_bottom());
    }
    #[test]
    fn test_will_colide_with_block() {
        let mut game = Game::new(Size {
            height: 4,
            width: 4,
        });

        game.board = game.board.replacing_figure_at_xy(0, 3, Some(FigureType::T));
        game.board = game.board.replacing_figure_at_xy(1, 3, Some(FigureType::T));
        game.board = game.board.replacing_figure_at_xy(2, 3, Some(FigureType::T));
        game.board = game.board.replacing_figure_at_xy(3, 3, Some(FigureType::T));

        assert!(!game.will_colide_with_block());
        game.move_active_figure_down();
        assert!(game.will_colide_with_block());
    }
    fn draw_to_cartesian(draw: Vec<Block>) -> Vec<Point> {
        return draw.iter().map(|block| block.position()).collect();
    }
    fn get_game() -> Game {
        return Game::new(Size {
            height: 40,
            width: 20,
        });
    }
}
