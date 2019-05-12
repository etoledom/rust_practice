use super::figure::{Figure, FigureType};
pub use utilities::block::Block;
use utilities::geometry::Point;
extern crate utilities;
use utilities::graphics::Color;

#[derive(Debug, Clone, PartialEq)]
pub struct ActiveFigure {
    figure: Figure,
    position: Point,
}

impl ActiveFigure {
    pub fn new(figure_type: FigureType, position: Point) -> ActiveFigure {
        return ActiveFigure {
            figure: Figure::new(figure_type),
            position,
        };
    }

    pub fn to_cartesian(&self) -> Vec<Point> {
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

    pub fn color(&self) -> Color {
        return self.figure.color();
    }

    pub fn position(&self) -> Point {
        return self.position;
    }

    pub fn left_edge(&self) -> i32 {
        let points = self.to_cartesian();
        return points.iter().fold(i32::max_value(), |edge, point| {
            if point.x < edge {
                return point.x;
            }
            return edge;
        });
    }

    pub fn right_edge(&self) -> i32 {
        let points = self.to_cartesian();
        return points.iter().fold(i32::min_value(), |edge, point| {
            if point.x > edge {
                return point.x;
            }
            return edge;
        });
    }

    pub fn updating_position_by_xy(&self, x: i32, y: i32) -> ActiveFigure {
        return ActiveFigure {
            figure: self.figure.clone(),
            position: Point {
                x: self.position().x + x,
                y: self.position().y + y,
            },
        };
    }

    pub fn rotated(&self) -> ActiveFigure {
        let figure = self.figure.rotated();
        return ActiveFigure {
            figure,
            position: self.position,
        };
    }
}

#[cfg(test)]
mod active_figure_tests {
    use super::*;
    #[test]
    fn test_to_cartesian_shifted() {
        let figure = ActiveFigure::new(FigureType::O, Point { x: 5, y: 5 });
        let coordinates = figure.to_cartesian();
        let expectation = vec![
            Point { x: 5, y: 5 },
            Point { x: 6, y: 5 },
            Point { x: 5, y: 6 },
            Point { x: 6, y: 6 },
        ];
        assert_eq!(coordinates, expectation);
    }
    #[test]
    fn test_color() {
        let figure_type = FigureType::T;
        let figure = ActiveFigure::new(FigureType::T, Point { x: 0, y: 0 });
        assert_eq!(figure.color(), figure_type.color());
    }
    #[test]
    fn test_update_position() {
        let figure = ActiveFigure::new(FigureType::L, Point { x: 0, y: 0 });
        let moved = figure.updating_position_by_xy(5, 5);
        assert_eq!(moved.position(), Point { x: 5, y: 5 });
    }
    #[test]
    fn test_left_edge() {
        let figure = ActiveFigure::new(FigureType::L, Point { x: 2, y: 2 });
        let edge = figure.left_edge();
        assert_eq!(edge, 2);
        let rotated_edge = figure.rotated().left_edge();
        assert_eq!(rotated_edge, 3);
    }
    #[test]
    fn test_right_edge() {
        let figure = ActiveFigure::new(FigureType::I, Point { x: 2, y: 2 });
        let edge = figure.right_edge();
        assert_eq!(edge, 5);
        let rotated_edge = figure.rotated().right_edge();
        assert_eq!(rotated_edge, 4);
    }
}
