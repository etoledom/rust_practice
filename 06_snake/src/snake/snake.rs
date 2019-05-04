// extern crate utilities;
use std::collections::LinkedList;
use utilities::block::Block;
use utilities::geometry::{Direction, Point};
use utilities::graphics::Color;

const SNAKE_COLOR: Color = Color {
    red: 0.0,
    green: 0.0,
    blue: 1.0,
    alpha: 1.0,
};

const SNAKE_START_DIRECTION: Direction = Direction::Right;

#[derive(Debug)]
pub struct Snake {
    pub direction: Direction,
    body: LinkedList<Block>,
    tail: Option<Block>,
}

fn snake_block_at(x: u32, y: u32) -> Block {
    return Block::new(x, y, 1, 1, SNAKE_COLOR);
}

impl Snake {
    pub fn new(origin: Point) -> Snake {
        let mut body = LinkedList::new();
        let (x, y) = (origin.x, origin.y);

        body.push_front(snake_block_at(x - 2, y));
        body.push_front(snake_block_at(x - 1, y));
        body.push_front(snake_block_at(x, y));

        return Snake {
            direction: SNAKE_START_DIRECTION,
            body,
            tail: None,
        };
    }

    pub fn draw(&self) -> Vec<Block> {
        let mut vector: Vec<Block> = Vec::new();
        for block in &self.body {
            vector.push(block.clone());
        }
        return vector;
    }

    pub fn head_position(&self) -> Point {
        let head_block = self.body.front().unwrap();
        return Point {
            x: head_block.position().x,
            y: head_block.position().y,
        };
    }

    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        return self.body.len();
    }

    pub fn move_forward(&mut self, direction: Option<Direction>) {
        match direction {
            Some(value) => self.direction = value,
            None => (),
        }

        let last = self.head_position();
        let new_block = match self.direction {
            Direction::Up => snake_block_at(last.x, last.y - 1),
            Direction::Down => snake_block_at(last.x, last.y + 1),
            Direction::Left => snake_block_at(last.x - 1, last.y),
            Direction::Right => snake_block_at(last.x + 1, last.y),
        };

        self.body.push_front(new_block);
        let removed_block = self.body.pop_back().unwrap();
        self.tail = Some(removed_block);
    }

    #[allow(dead_code)]
    pub fn head_direction(&self) -> Direction {
        return self.direction;
    }

    pub fn next_head(&self, direction: Option<Direction>) -> Point {
        let head = self.head_position();
        let moving_dir = match direction {
            Some(value) => value,
            None => self.direction,
        };

        let (x, y) = match moving_dir {
            Direction::Up => (head.x, head.y - 1),
            Direction::Down => (head.x, head.y + 1),
            Direction::Left => (head.x - 1, head.y),
            Direction::Right => (head.x + 1, head.y),
        };

        return Point { x, y };
    }

    pub fn restore_tail(&mut self) {
        let block = self.tail.clone().unwrap();
        self.body.push_back(block);
    }

    pub fn is_overlaping(&self, point: &Point) -> bool {
        return self.body.iter().fold(false, |acc, block| {
            acc || (block.position().x == point.x && block.position().y == point.y)
        });
    }
}

#[cfg(test)]
mod snake_tests {
    use super::*;

    #[test]
    fn test_snake_start_at_position() {
        let start_point = Point { x: 2, y: 2 };
        let snake = Snake::new(start_point);

        assert_eq!(snake.direction, SNAKE_START_DIRECTION);
        assert_eq!(snake.head_position(), start_point);
    }
    #[test]
    fn test_move_forward_right() {
        let mut snake = get_snake_at(2, 2);
        snake.move_forward(Some(Direction::Right));
        assert_eq!(snake.head_position(), Point { x: 3, y: 2 });
    }
    #[test]
    fn test_move_forward_down() {
        let mut snake = get_snake_at(2, 2);
        snake.move_forward(Some(Direction::Down));
        assert_eq!(snake.head_position(), Point { x: 2, y: 3 });
    }
    #[test]
    fn test_move_forward_left() {
        let mut snake = get_snake_at(2, 2);
        snake.move_forward(Some(Direction::Left));
        assert_eq!(snake.head_position(), Point { x: 1, y: 2 });
    }
    #[test]
    fn test_move_forward_up() {
        let mut snake = get_snake_at(2, 2);
        snake.move_forward(Some(Direction::Up));
        assert_eq!(snake.head_position(), Point { x: 2, y: 1 });
    }
    #[test]
    fn test_next_head_right() {
        let snake = get_snake_at(2, 2);
        assert_eq!(
            snake.next_head(Some(Direction::Right)),
            Point { x: 3, y: 2 }
        );
    }
    #[test]
    fn test_next_head_down() {
        let snake = get_snake_at(2, 2);
        assert_eq!(snake.next_head(Some(Direction::Down)), Point { x: 2, y: 3 });
    }
    #[test]
    fn test_next_head_left() {
        let snake = get_snake_at(2, 2);
        assert_eq!(snake.next_head(Some(Direction::Left)), Point { x: 1, y: 2 });
    }
    #[test]
    fn test_next_head_up() {
        let snake = get_snake_at(2, 2);
        assert_eq!(snake.next_head(Some(Direction::Up)), Point { x: 2, y: 1 });
    }
    #[test]
    fn test_restore_tail() {
        let mut snake = get_snake_at(2, 2);
        snake.move_forward(None);
        snake.restore_tail();
        let drawed_snake = snake.draw();
        assert_eq!(drawed_snake.len(), 4);
        assert_eq!(*drawed_snake.last().unwrap(), snake_block_at(0, 2));
    }
    #[test]
    fn test_is_overlaping() {
        let mut snake = get_snake_at(5, 5);
        let dead_point = Point { x: 4, y: 4 };
        assert!(!snake.is_overlaping(&dead_point));

        snake.move_forward(None); // move to (6,5)
        assert!(!snake.is_overlaping(&dead_point));

        snake.move_forward(Some(Direction::Up)); // move to (6,4)
        assert!(!snake.is_overlaping(&dead_point));

        snake.move_forward(Some(Direction::Left)); // move to (5,4)
        assert!(!snake.is_overlaping(&dead_point));

        snake.move_forward(None); // move to (4,4)
        assert!(snake.is_overlaping(&dead_point)); // is DEAD!
    }

    fn get_snake_at(x: u32, y: u32) -> Snake {
        let start_point = Point { x, y };
        return Snake::new(start_point);
    }
}
