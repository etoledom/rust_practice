#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UPoint {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Size {
    pub height: usize,
    pub width: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Rect {
    pub origin: Point,
    pub size: Size,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[cfg(test)]
mod geometry_tests {
    #[test]
    fn test_direction_opposite() {
        assert_eq!(super::Direction::Up.opposite(), super::Direction::Down);
        assert_eq!(super::Direction::Down.opposite(), super::Direction::Up);
        assert_eq!(super::Direction::Left.opposite(), super::Direction::Right);
        assert_eq!(super::Direction::Right.opposite(), super::Direction::Left);
    }
}
