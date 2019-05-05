mod figure;
mod figure_type;
mod matrix;
extern crate utilities;
use figure::Figure;
use figure_type::FigureType;
pub use utilities::block::Block;
use utilities::geometry::{Point, Rect, Size};
use utilities::graphics::Color;

const MOVING_PERIOD: f64 = 0.2; //secs
const BACKGROUND_COLOR: Color = Color {
    red: 1.0,
    green: 1.0,
    blue: 1.0,
    alpha: 1.0,
};

#[derive(Debug, Clone, PartialEq)]
struct ActiveFigure {
    figure: Figure,
    position: Point,
}

impl ActiveFigure {
    fn to_cartesian(&self) -> Vec<Point> {
        let figure_points = self.figure.to_cartesian();
        let (dx, dy) = (self.position.x, self.position.y);
        return figure_points
            .iter()
            .map(|point| Point {
                x: point.x + dx,
                y: point.y + dy,
            })
            .collect();
    }
}

struct Game {
    board: Vec<Vec<Option<FigureType>>>,
    points: u128,
    active: ActiveFigure,
    waiting_time: f64,
}

impl Game {
    fn new(size: Size) -> Game {
        let mut board = vec![];
        for _y in 0..size.height {
            let mut line: Vec<Option<FigureType>> = vec![];
            for _x in 0..size.width {
                line.push(None);
            }
            board.push(line);
        }

        let active = ActiveFigure {
            figure: Figure::new(FigureType::T),
            position: Point { x: 0, y: 0 },
        };

        return Game {
            board,
            points: 0,
            active,
            waiting_time: 0.0,
        };
    }

    pub fn draw(&self) -> Vec<Block> {
        let mut blocks = vec![];
        for y in 0..self.board.len() {
            for x in 0..self.board[y].len() {
                if let Some(square) = &self.board[y][x] {
                    let block = Block::new(x as u32, y as u32, 1, 1, square.color());
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
            acc || point.y == ((self.board.len() - 1) as u32)
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
        return self.board[point.y as usize][point.x as usize].is_some();
    }

    fn active_figure_moved_down(&self) -> ActiveFigure {
        return ActiveFigure {
            figure: self.active.figure.clone(),
            position: Point {
                x: self.active.position.x,
                y: self.active.position.y + 1,
            },
        };
    }

    fn move_active_figure_down(&mut self) {
        let new_active = self.active_figure_moved_down();
        self.update_active_with(new_active);
    }

    fn rotate_active_figure(&mut self) {
        let figure = self.active.figure.rotated();
        let new_active = ActiveFigure {
            figure,
            position: self.active.position,
        };
        self.update_active_with(new_active);
    }

    fn update_active_with(&mut self, new_active: ActiveFigure) {
        self.remove_active_figure_from_board();
        self.active = new_active;
        self.add_active_figure_from_board();
    }

    fn remove_active_figure_from_board(&mut self) {
        let positions = self.active.to_cartesian();
        for point in positions {
            self.board[point.y as usize][point.x as usize] = None;
        }
    }

    fn add_active_figure_from_board(&mut self) {
        let positions = self.active.to_cartesian();
        for point in positions {
            self.board[point.y as usize][point.x as usize] = Some(self.active.figure.get_type());
        }
    }
}

#[cfg(test)]
mod game_tests {
    use super::*;
    #[test]
    fn test_active_figure_is_added_to_the_board() {
        let mut game = get_game();
        let active_points = game.active.to_cartesian();

        game.add_active_figure_from_board();
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
        let rotated = game.active.figure.rotated();
        let rotated_active = ActiveFigure {
            figure: rotated,
            position: game.active.position,
        };
        game.rotate_active_figure();
        let drawed_points = draw_to_cartesian(game.draw());
        assert_eq!(drawed_points, rotated_active.to_cartesian());
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
        let mut game = get_game();
        game.board = vec![
            vec![None, None, None, None],
            vec![None, None, None, None],
            vec![None, None, None, None],
            vec![
                Some(FigureType::I),
                Some(FigureType::I),
                Some(FigureType::I),
                Some(FigureType::I),
            ],
        ];
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
