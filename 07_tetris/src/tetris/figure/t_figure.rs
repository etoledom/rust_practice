// use super::figure::Figure;
// use super::matrix::Matrix;
// use utilities::block::Block;
// use utilities::geometry::{Point, Rect, Size};
// use utilities::graphics::Color;

// const COLOR: Color = Color {
//     red: 0.0,
//     green: 0.0,
//     blue: 0.0,
//     alpha: 1.0,
// };

// pub struct T_Figure {
//     color: Color,
//     matrix: Matrix,
// }

// impl T_Figure {
//     pub fn new() -> T_Figure {
//         let matrix = Matrix {
//             data: vec![
//                 vec![1, 1, 1], //Avoid lint
//                 vec![0, 1, 0],
//                 vec![0, 0, 0],
//             ],
//         };
//         return T_Figure {
//             color: COLOR,
//             matrix,
//         };
//     }

//     fn block_at_xy(&self, x: u32, y: u32) -> Block {
//         return Block {
//             rect: Rect {
//                 origin: Point { x, y },
//                 size: Size {
//                     height: 1,
//                     width: 1,
//                 },
//             },
//             color: COLOR,
//         };
//     }
// }

// impl Figure for T_Figure {
//     fn rotated(&self) -> Self {
//         let mut data = vec![];
//         for i in 0..=2 {
//             let mut vec = vec![];
//             for j in 0..=2 {
//                 vec.push(self.matrix.data[j][2 - i]);
//             }
//             data.push(vec);
//         }

//         return T_Figure {
//             matrix: Matrix { data },
//             color: COLOR,
//         };
//     }

//     fn draw(&self) -> Vec<Block> {
//         let mut blocks = vec![];
//         for y in 0..=2 {
//             for x in 0..=2 {
//                 if self.matrix.data[y][x] == 1 {
//                     blocks.push(self.block_at_xy(x as u32, y as u32));
//                 }
//             }
//         }
//         return blocks;
//     }
// }

// #[cfg(test)]
// mod t_figure_tests {
//     use super::*;
//     #[test]
//     fn test_rotation() {
//         let figure = T_Figure::new();
//         let rotated = figure.rotated();
//         let rotated_matrix = Matrix {
//             data: vec![
//                 vec![1, 0, 0], //Avoid lint
//                 vec![1, 1, 0],
//                 vec![1, 0, 0],
//             ],
//         };
//         assert_eq!(rotated.matrix.data, rotated_matrix.data);
//     }
//     #[test]
//     fn test_double_rotation() {
//         let figure = T_Figure::new();
//         let rotated = figure.rotated().rotated();
//         let rotated_matrix = Matrix {
//             data: vec![
//                 vec![0, 0, 0], //Avoid lint
//                 vec![0, 1, 0],
//                 vec![1, 1, 1],
//             ],
//         };
//         assert_eq!(rotated.matrix.data, rotated_matrix.data);
//     }
//     #[test]
//     fn test_triple_rotation() {
//         let figure = T_Figure::new();
//         let rotated = figure.rotated().rotated().rotated();
//         let rotated_matrix = Matrix {
//             data: vec![
//                 vec![0, 0, 1], //Avoid lint
//                 vec![0, 1, 1],
//                 vec![0, 0, 1],
//             ],
//         };
//         assert_eq!(rotated.matrix.data, rotated_matrix.data);
//     }
//     #[test]
//     fn test_full_rotation_loop() {
//         let figure = T_Figure::new();
//         let initial_matrix = figure.matrix.data.clone();
//         let rotated = figure.rotated().rotated().rotated().rotated();
//         assert_eq!(rotated.matrix.data, initial_matrix);
//     }
//     #[test]
//     fn test_draw() {
//         let figure = T_Figure::new();
//         let drawed = figure.draw();
//         assert_eq!(drawed.len(), 4);
//         assert_eq!(drawed[0].position(), Point { x: 0, y: 0 });
//         assert_eq!(drawed[1].position(), Point { x: 1, y: 0 });
//         assert_eq!(drawed[2].position(), Point { x: 2, y: 0 });
//         assert_eq!(drawed[3].position(), Point { x: 1, y: 1 });
//     }
// }
