use super::figure_type::FigureType;
use super::matrix::Matrix;
use utilities::block::Block;
use utilities::geometry::{Point, Rect, Size};
use utilities::graphics::Color;

#[derive(Debug, Clone, PartialEq)]
pub struct Figure {
    figure_type: FigureType,
    matrix: Matrix,
}

impl Figure {
    pub fn new(figure_type: FigureType) -> Figure {
        let matrix = figure_type.initial_matrix();
        return Figure {
            figure_type,
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
        let matrix_length = self.matrix.data.len();
        for i in 0..matrix_length {
            let mut vec = vec![];
            for j in 0..matrix_length {
                vec.push(self.matrix.data[(matrix_length - 1) - j][i]);
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
    fn test_T_figure_rotation() {
        let figure = Figure::new(FigureType::T);
        let first_rotation = figure.rotated();
        let second_rotation = first_rotation.rotated();
        let third_rotation = second_rotation.rotated();
        let full_loop_rotation = third_rotation.rotated();
        let first_rotation_matrix = Matrix {
            data: vec![vec![0, 1, 0], vec![0, 1, 1], vec![0, 1, 0]],
        };
        let second_rotation_matrix = Matrix {
            data: vec![vec![0, 0, 0], vec![1, 1, 1], vec![0, 1, 0]],
        };
        let third_rotation_matrix = Matrix {
            data: vec![vec![0, 1, 0], vec![1, 1, 0], vec![0, 1, 0]],
        };
        let full_loop_rotation_matrix = Matrix {
            data: vec![vec![0, 1, 0], vec![1, 1, 1], vec![0, 0, 0]],
        };
        assert_eq!(first_rotation.matrix.data, first_rotation_matrix.data);
        assert_eq!(second_rotation.matrix.data, second_rotation_matrix.data);
        assert_eq!(third_rotation.matrix.data, third_rotation_matrix.data);
        assert_eq!(
            full_loop_rotation.matrix.data,
            full_loop_rotation_matrix.data
        );
    }
    #[test]
    fn test_I_figure_rotation() {
        let figure = Figure::new(FigureType::I);
        let first_rotation = figure.rotated();
        let second_rotation = first_rotation.rotated();
        let third_rotation = second_rotation.rotated();
        let full_loop_rotation = third_rotation.rotated();
        let first_rotation_matrix = Matrix {
            data: vec![
                vec![0, 0, 1, 0],
                vec![0, 0, 1, 0],
                vec![0, 0, 1, 0],
                vec![0, 0, 1, 0],
            ],
        };
        let second_rotation_matrix = Matrix {
            data: vec![
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
                vec![1, 1, 1, 1],
                vec![0, 0, 0, 0],
            ],
        };
        let third_rotation_matrix = Matrix {
            data: vec![
                vec![0, 1, 0, 0],
                vec![0, 1, 0, 0],
                vec![0, 1, 0, 0],
                vec![0, 1, 0, 0],
            ],
        };
        let full_loop_rotation_matrix = Matrix {
            data: vec![
                vec![0, 0, 0, 0],
                vec![1, 1, 1, 1],
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
            ],
        };
        assert_eq!(first_rotation.matrix.data, first_rotation_matrix.data);
        assert_eq!(second_rotation.matrix.data, second_rotation_matrix.data);
        assert_eq!(third_rotation.matrix.data, third_rotation_matrix.data);
        assert_eq!(
            full_loop_rotation.matrix.data,
            full_loop_rotation_matrix.data
        );
    }
    #[test]
    fn test_L_figure_rotation() {
        let figure = Figure::new(FigureType::L);
        let first_rotation = figure.rotated();
        let second_rotation = first_rotation.rotated();
        let third_rotation = second_rotation.rotated();
        let full_loop_rotation = third_rotation.rotated();
        let first_rotation_matrix = Matrix {
            data: vec![vec![0, 1, 0], vec![0, 1, 0], vec![0, 1, 1]],
        };
        let second_rotation_matrix = Matrix {
            data: vec![vec![0, 0, 0], vec![1, 1, 1], vec![1, 0, 0]],
        };
        let third_rotation_matrix = Matrix {
            data: vec![vec![1, 1, 0], vec![0, 1, 0], vec![0, 1, 0]],
        };
        let full_loop_rotation_matrix = Matrix {
            data: vec![vec![0, 0, 1], vec![1, 1, 1], vec![0, 0, 0]],
        };
        assert_eq!(first_rotation.matrix.data, first_rotation_matrix.data);
        assert_eq!(second_rotation.matrix.data, second_rotation_matrix.data);
        assert_eq!(third_rotation.matrix.data, third_rotation_matrix.data);
        assert_eq!(
            full_loop_rotation.matrix.data,
            full_loop_rotation_matrix.data
        );
    }
    #[test]
    fn test_J_figure_rotation() {
        let figure = Figure::new(FigureType::J);
        let first_rotation = figure.rotated();
        let second_rotation = first_rotation.rotated();
        let third_rotation = second_rotation.rotated();
        let full_loop_rotation = third_rotation.rotated();
        let first_rotation_matrix = Matrix {
            data: vec![vec![0, 1, 1], vec![0, 1, 0], vec![0, 1, 0]],
        };
        let second_rotation_matrix = Matrix {
            data: vec![vec![0, 0, 0], vec![1, 1, 1], vec![0, 0, 1]],
        };
        let third_rotation_matrix = Matrix {
            data: vec![vec![0, 1, 0], vec![0, 1, 0], vec![1, 1, 0]],
        };
        let full_loop_rotation_matrix = Matrix {
            data: vec![vec![1, 0, 0], vec![1, 1, 1], vec![0, 0, 0]],
        };
        assert_eq!(first_rotation.matrix.data, first_rotation_matrix.data);
        assert_eq!(second_rotation.matrix.data, second_rotation_matrix.data);
        assert_eq!(third_rotation.matrix.data, third_rotation_matrix.data);
        assert_eq!(
            full_loop_rotation.matrix.data,
            full_loop_rotation_matrix.data
        );
    }
    #[test]
    fn test_S_figure_rotation() {
        let figure = Figure::new(FigureType::S);
        let first_rotation = figure.rotated();
        let second_rotation = first_rotation.rotated();
        let third_rotation = second_rotation.rotated();
        let full_loop_rotation = third_rotation.rotated();
        let first_rotation_matrix = Matrix {
            data: vec![vec![0, 1, 0], vec![0, 1, 1], vec![0, 0, 1]],
        };
        let second_rotation_matrix = Matrix {
            data: vec![vec![0, 0, 0], vec![0, 1, 1], vec![1, 1, 0]],
        };
        let third_rotation_matrix = Matrix {
            data: vec![vec![1, 0, 0], vec![1, 1, 0], vec![0, 1, 0]],
        };
        let full_loop_rotation_matrix = Matrix {
            data: vec![vec![0, 1, 1], vec![1, 1, 0], vec![0, 0, 0]],
        };
        assert_eq!(first_rotation.matrix.data, first_rotation_matrix.data);
        assert_eq!(second_rotation.matrix.data, second_rotation_matrix.data);
        assert_eq!(third_rotation.matrix.data, third_rotation_matrix.data);
        assert_eq!(
            full_loop_rotation.matrix.data,
            full_loop_rotation_matrix.data
        );
    }
    #[test]
    fn test_Z_figure_rotation() {
        let figure = Figure::new(FigureType::Z);
        let first_rotation = figure.rotated();
        let second_rotation = first_rotation.rotated();
        let third_rotation = second_rotation.rotated();
        let full_loop_rotation = third_rotation.rotated();
        let first_rotation_matrix = Matrix {
            data: vec![vec![0, 0, 1], vec![0, 1, 1], vec![0, 1, 0]],
        };
        let second_rotation_matrix = Matrix {
            data: vec![vec![0, 0, 0], vec![1, 1, 0], vec![0, 1, 1]],
        };
        let third_rotation_matrix = Matrix {
            data: vec![vec![0, 1, 0], vec![1, 1, 0], vec![1, 0, 0]],
        };
        let full_loop_rotation_matrix = Matrix {
            data: vec![vec![1, 1, 0], vec![0, 1, 1], vec![0, 0, 0]],
        };
        assert_eq!(first_rotation.matrix.data, first_rotation_matrix.data);
        assert_eq!(second_rotation.matrix.data, second_rotation_matrix.data);
        assert_eq!(third_rotation.matrix.data, third_rotation_matrix.data);
        assert_eq!(
            full_loop_rotation.matrix.data,
            full_loop_rotation_matrix.data
        );
    }
    #[test]
    fn test_O_figure_rotation() {
        let figure = Figure::new(FigureType::O);
        let first_rotation = figure.rotated();
        let second_rotation = first_rotation.rotated();
        let third_rotation = second_rotation.rotated();
        let full_loop_rotation = third_rotation.rotated();
        let how_it_should_always_look_like = Matrix {
            data: vec![vec![1, 1], vec![1, 1]],
        };
        assert_eq!(
            first_rotation.matrix.data,
            how_it_should_always_look_like.data
        );
        assert_eq!(
            second_rotation.matrix.data,
            how_it_should_always_look_like.data
        );
        assert_eq!(
            third_rotation.matrix.data,
            how_it_should_always_look_like.data
        );
        assert_eq!(
            full_loop_rotation.matrix.data,
            how_it_should_always_look_like.data
        );
    }
    #[test]
    fn test_draw() {
        let figure = Figure::new(FigureType::T);
        let drawed = figure.to_cartesian();
        assert_eq!(drawed.len(), 4);
        assert_eq!(drawed[0], Point { x: 1, y: 0 });
        assert_eq!(drawed[1], Point { x: 0, y: 1 });
        assert_eq!(drawed[2], Point { x: 1, y: 1 });
        assert_eq!(drawed[3], Point { x: 2, y: 1 });
    }
}
