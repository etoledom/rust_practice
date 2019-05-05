mod figure;
mod figure_type;
mod matrix;
extern crate utilities;
use figure::Figure;
use figure_type::FigureType;
pub use utilities::block::Block;
use utilities::geometry::{Point, Rect, Size};
use utilities::graphics::Color;

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
}

impl Game {
    fn new(size: Size) -> Game {
        let mut board = vec![];
        for _y in 0..=size.height {
            let mut line: Vec<Option<FigureType>> = vec![];
            for _x in 0..=size.width {
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

    fn move_active_figure_down(&mut self) {
        let new_active = ActiveFigure {
            figure: self.active.figure.clone(),
            position: Point {
                x: self.active.position.x,
                y: self.active.position.y + 1,
            },
        };
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
