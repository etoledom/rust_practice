use super::{ActiveFigure, Block, Board, FigureType, Point, Size};

const MOVING_PERIOD: f64 = 0.2; //secs

pub struct Game {
    board: Board,
    points: u128,
    active: ActiveFigure,
    waiting_time: f64,
}

impl Game {
    pub fn new(size: Size) -> Game {
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
                    let block = Block::new(x as i32, y as i32, 1, 1, square.color());
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

    pub fn rotate(&mut self) {
        self.rotate_active_figure();
    }

    pub fn move_left(&mut self) {
        self.update_active_with(self.active_figure_moved_left());
    }

    pub fn move_right(&mut self) {
        self.update_active_with(self.active_figure_moved_right());
    }

    pub fn move_down(&mut self) {
        self.update_active_with(self.active_figure_moved_down());
    }

    fn has_valid_position(&self, active_figure: &ActiveFigure) -> bool {
        return !self.will_colide_with_block(active_figure)
            && !self.will_collide_with_edge(active_figure);
    }

    fn will_collide_with_edge(&self, active_figure: &ActiveFigure) -> bool {
        let collided_with_left = active_figure.left_edge() < 0;
        let collided_with_right = active_figure.right_edge() >= self.board.width() as i32;
        let collided_with_bottom = active_figure.bottom_edge() >= self.board.height() as i32;
        return collided_with_left || collided_with_right || collided_with_bottom;
    }

    fn update_game(&mut self) {
        if self.can_move_down() {
            self.move_down();
        } else {
            self.add_active_figure_to_board();
            let completed_lines_count = self.remove_completed_lines();
            self.add_points_for(completed_lines_count);
            self.add_new_active_figure();
        }
    }

    fn can_move_down(&self) -> bool {
        let moved_down = self.active_figure_moved_down();
        return !self.is_at_the_bottom() && !self.will_colide_with_block(&moved_down);
    }

    fn is_at_the_bottom(&self) -> bool {
        return self.active.bottom_edge() == (self.board.height() as i32 - 1);
    }

    fn will_colide_with_block(&self, figure: &ActiveFigure) -> bool {
        let points = figure.to_cartesian();
        for point in points {
            if self.board.contains(point) {
                return true;
            }
        }
        return false;
    }

    fn active_figure_moved_down(&self) -> ActiveFigure {
        return self.active.updating_position_by_xy(0, 1);
    }

    fn active_figure_moved_left(&self) -> ActiveFigure {
        return self.active.updating_position_by_xy(-1, 0);
    }

    fn active_figure_moved_right(&self) -> ActiveFigure {
        return self.active.updating_position_by_xy(1, 0);
    }

    fn rotate_active_figure(&mut self) {
        if let Some(rotated) = self.wall_kicked_rotated_active_figure() {
            self.update_active_with(rotated);
        }
    }

    fn wall_kicked_rotated_active_figure(&self) -> Option<ActiveFigure> {
        let wall_kick_tests = self.active.wall_kick_tests();
        for test in wall_kick_tests {
            let moved_figure = self.active.updating_position_by_xy(test.x, test.y);
            let test_figure = moved_figure.rotated();
            if self.has_valid_position(&test_figure) {
                return Some(test_figure);
            }
        }
        return None;
    }

    fn update_active_with(&mut self, new_active: ActiveFigure) {
        if self.has_valid_position(&new_active) {
            self.active = new_active;
        }
    }

    fn add_active_figure_to_board(&mut self) {
        for point in self.active.to_cartesian() {
            self.board = self.board.replacing_figure_at_xy(
                point.x as usize,
                point.y as usize,
                Some(self.active.get_type()),
            );
        }
    }

    fn add_new_active_figure(&mut self) {
        let new_active = ActiveFigure::new(FigureType::I, Point { x: 0, y: 0 });
        self.update_active_with(new_active);
    }

    fn remove_completed_lines(&mut self) -> usize {
        let lines = self.lines_completed();
        self.board = self.board.removing_lines(&lines);
        return lines.len();
    }

    fn lines_completed(&self) -> Vec<usize> {
        let mut completed_lines: Vec<usize> = vec![];
        for line_number in 0..self.board.height() {
            if self.is_line_completed(line_number) {
                completed_lines.push(line_number);
            }
        }
        return completed_lines;
    }

    fn is_line_completed(&self, line_number: usize) -> bool {
        if let Some(line) = self.board.get_line(line_number) {
            return !line.contains(&None);
        }
        return false;
    }

    fn add_points_for(&mut self, completed_lines: usize) {
        self.points += (completed_lines as u128) * 100;
    }
}

#[cfg(test)]
mod game_tests {
    use super::*;
    #[test]
    fn test_active_figure_is_draw() {
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
        game.move_down();
        let drawed_points = draw_to_cartesian(game.draw());

        assert_eq!(drawed_points, expected);
    }
    #[test]
    fn test_active_figure_does_not_move_lower_than_floor() {
        let mut game = get_game();
        let y = game.board.height() as i32 - 3; // 3 spaces before the floor
        game.active = ActiveFigure::new(FigureType::O, Point { x: 10, y });
        game.move_down();
        game.move_down();
        game.move_down();
        game.move_down();
        assert!(game.is_at_the_bottom());
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
        game.move_down();
        assert!(!game.is_at_the_bottom());
        game.move_down();
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

        let colider = ActiveFigure::new(FigureType::I, Point { x: 0, y: 0 });
        let rotated = colider.rotated();

        assert!(!game.will_colide_with_block(&game.active));
        assert!(game.will_colide_with_block(&rotated));
    }

    #[test]
    fn test_move_left() {
        let mut game = get_game();
        game.active = ActiveFigure::new(FigureType::L, Point { x: 10, y: 0 });
        assert_eq!(game.active.left_edge(), 10);
        game.move_left();
        assert_eq!(game.active.left_edge(), 9);
    }
    #[test]
    fn test_move_left_does_not_go_beyond_zero() {
        let mut game = get_game();
        game.active = ActiveFigure::new(FigureType::L, Point { x: 2, y: 0 });
        game.active = game.active.rotated(); // left edge is now at x: 3
        assert_eq!(game.active.left_edge(), 3);
        game.move_left(); // x: 2
        game.move_left(); // x: 1
        game.move_left(); // x: 0
        game.move_left(); // x: 0
        assert_eq!(game.active.left_edge(), 0);
    }
    #[test]
    fn test_move_right() {
        let mut game = get_game();
        game.active = ActiveFigure::new(FigureType::L, Point { x: 0, y: 0 });
        game.move_right();
        assert_eq!(game.active.position(), Point { x: 1, y: 0 });
    }
    #[test]
    fn test_move_right_does_not_go_beyond_board_edge() {
        let mut game = get_game();
        game.active = ActiveFigure::new(FigureType::I, Point { x: 16, y: 0 });
        game.active = game.active.rotated(); // right edge is now at 18
        assert_eq!(game.active.left_edge(), 18);
        game.move_right(); // x: 19
        game.move_right(); // x: 19
        assert_eq!(game.active.right_edge(), 19);
    }
    #[test]
    fn test_add_active_figure_to_board() {
        let mut game = get_game();
        assert!(game.draw_board().is_empty());
        game.add_active_figure_to_board();
        assert_eq!(game.draw_board().len(), 4);
    }
    #[test]
    fn test_active_figure_is_added_when_it_touches_the_floor() {
        let mut game = Game::new(Size {
            height: 4,
            width: 10,
        });
        assert_eq!(game.active.position().y, 0); // lowest figure block is at y: 1
        assert!(game.draw_board().is_empty());
        game.update(10.0); // y: 2
        game.update(10.0); // y: 3
        game.update(10.0); // -> Should add figure to board and create new active

        assert_eq!(game.active.position().y, 0);
        assert_eq!(game.draw_board().len(), 4);
    }
    #[test]
    fn test_active_figure_is_added_when_touches_block() {
        let mut game = Game::new(Size {
            height: 7,
            width: 10,
        });
        game.active = ActiveFigure::new(FigureType::T, Point { x: 0, y: 5 });
        game.update(10.0); // current figure should be added to the board
        assert_eq!(game.draw_board().len(), 4); // Next figure should colide at y: 5

        game.update(10.0); // y: 2
        game.update(10.0); // y: 3
        game.update(10.0); // y: 4
        game.update(10.0); // y: 5

        assert_eq!(game.active.position().y, 0);
        assert_eq!(game.draw_board().len(), 8);
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
