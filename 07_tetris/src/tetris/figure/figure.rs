use super::matrix::Matrix;
use utilities::block::Block;
use utilities::geometry::{Point, Rect, Size};
use utilities::graphics::Color;

const COLOR: Color = Color {
    red: 0.0,
    green: 0.0,
    blue: 0.0,
    alpha: 1.0,
};

#[derive(Debug, Clone, PartialEq)]
pub enum FigureType {
    I,
    T,
    L,
    J,
    O,
    Z,
    S,
}

impl FigureType {
    pub fn color(&self) -> Color {
        return Color {
            red: 1.0,
            green: 0.0,
            blue: 0.0,
            alpha: 1.0,
        };
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Figure {
    figure_type: FigureType,
    matrix: Matrix,
}

impl Figure {
    pub fn new() -> Figure {
        let matrix = Matrix {
            data: vec![
                vec![1, 1, 1], //Avoid lint
                vec![0, 1, 0],
                vec![0, 0, 0],
            ],
        };
        return Figure {
            figure_type: FigureType::T,
            matrix,
        };
    }

    pub fn get_type(&self) -> FigureType {
        return self.figure_type.clone();
    }

    pub fn color(&self) -> Color {
        return self.figure_type.color();
    }

    fn block_at_xy(&self, x: u32, y: u32) -> Block {
        return Block {
            rect: Rect {
                origin: Point { x, y },
                size: Size {
                    height: 1,
                    width: 1,
                },
            },
            color: self.figure_type.color(),
        };
    }

    pub fn rotated(&self) -> Self {
        let mut data = vec![];
        for i in 0..=2 {
            let mut vec = vec![];
            for j in 0..=2 {
                vec.push(self.matrix.data[j][2 - i]);
            }
            data.push(vec);
        }

        return Figure {
            matrix: Matrix { data },
            figure_type: self.figure_type.clone(),
        };
    }

    pub fn to_cartesian(&self) -> Vec<Point> {
        let mut points = vec![];
        for y in 0..=2 {
            for x in 0..=2 {
                if self.matrix.data[y][x] == 1 {
                    points.push(Point {
                        x: x as u32,
                        y: y as u32,
                    });
                }
            }
        }
        return points;
    }
}

#[cfg(test)]
mod figure_tests {
    use super::*;
    #[test]
    fn test_rotation() {
        let figure = Figure::new();
        let rotated = figure.rotated();
        let rotated_matrix = Matrix {
            data: vec![
                vec![1, 0, 0], //Avoid lint
                vec![1, 1, 0],
                vec![1, 0, 0],
            ],
        };
        assert_eq!(rotated.matrix.data, rotated_matrix.data);
    }
    #[test]
    fn test_double_rotation() {
        let figure = Figure::new();
        let rotated = figure.rotated().rotated();
        let rotated_matrix = Matrix {
            data: vec![
                vec![0, 0, 0], //Avoid lint
                vec![0, 1, 0],
                vec![1, 1, 1],
            ],
        };
        assert_eq!(rotated.matrix.data, rotated_matrix.data);
    }
    #[test]
    fn test_triple_rotation() {
        let figure = Figure::new();
        let rotated = figure.rotated().rotated().rotated();
        let rotated_matrix = Matrix {
            data: vec![
                vec![0, 0, 1], //Avoid lint
                vec![0, 1, 1],
                vec![0, 0, 1],
            ],
        };
        assert_eq!(rotated.matrix.data, rotated_matrix.data);
    }
    #[test]
    fn test_full_rotation_loop() {
        let figure = Figure::new();
        let initial_matrix = figure.matrix.data.clone();
        let rotated = figure.rotated().rotated().rotated().rotated();
        assert_eq!(rotated.matrix.data, initial_matrix);
    }
    #[test]
    fn test_draw() {
        let figure = Figure::new();
        let drawed = figure.to_cartesian();
        assert_eq!(drawed.len(), 4);
        assert_eq!(drawed[0], Point { x: 0, y: 0 });
        assert_eq!(drawed[1], Point { x: 1, y: 0 });
        assert_eq!(drawed[2], Point { x: 2, y: 0 });
        assert_eq!(drawed[3], Point { x: 1, y: 1 });
    }
}
