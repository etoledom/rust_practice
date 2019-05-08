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

    pub fn updating_position_by_xy(&self, x: u32, y: u32) -> ActiveFigure {
        return ActiveFigure {
            figure: self.figure.clone(),
            position: Point {
                x: self.position.x + x,
                y: self.position.y + y,
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
