use super::figure::Matrix;
use super::FigureType;
use super::Point;
use super::Size;

pub struct Board {
    matrix: Matrix<Option<FigureType>>,
}

impl Board {
    pub fn new(size: Size) -> Board {
        let mut cells = vec![];
        for _y in 0..size.height {
            let mut line: Vec<Option<FigureType>> = vec![];
            for _x in 0..size.width {
                line.push(None);
            }
            cells.push(line);
        }
        let matrix = Matrix::new(cells);
        return Board { matrix };
    }

    pub fn height(&self) -> usize {
        return self.matrix.height();
    }

    pub fn width(&self) -> usize {
        return self.matrix.width();
    }

    pub fn figure_at_xy(&self, x: usize, y: usize) -> &Option<FigureType> {
        if let Some(element) = self.matrix.at_xy(x, y) {
            return element;
        } else {
            return &None;
        }
    }

    pub fn replacing_figure_at_xy(
        &self,
        x: usize,
        y: usize,
        figure_type: Option<FigureType>,
    ) -> Board {
        let matrix = self.matrix.replacing_at_xy(x, y, figure_type);
        return Board { matrix };
    }

    pub fn contains(&self, point: Point) -> bool {
        if point.x < 0 || point.y < 0 {
            return false;
        }
        return self
            .figure_at_xy(point.x as usize, point.y as usize)
            .is_some();
    }
}

#[cfg(test)]
mod board_tests {
    use super::*;

    #[test]
    fn test_height() {
        let height = 10;
        let board = Board::new(Size { height, width: 10 });
        assert_eq!(board.height(), height as usize);
    }
    #[test]
    fn test_width() {
        let width = 10;
        let board = Board::new(Size { height: 10, width });
        assert_eq!(board.width(), width);
    }
    #[test]
    fn test_replacing_figure() {
        let board = Board::new(Size {
            height: 2,
            width: 1,
        });
        let replaced_board = board.replacing_figure_at_xy(0, 0, Some(FigureType::I));
        assert!(replaced_board.matrix.at_xy(0, 0).is_some());
    }
    #[test]
    fn test_does_not_contains() {
        let board = Board::new(Size {
            height: 4,
            width: 4,
        });
        assert!(!board.contains(Point { x: 0, y: 0 }));
        assert!(!board.contains(Point { x: 100, y: 100 }));
        let board_with_figure = board.replacing_figure_at_xy(0, 0, Some(FigureType::I));
        assert!(board_with_figure.contains(Point { x: 0, y: 0 }));
    }
    #[test]
    fn test_contains() {
        let board = Board::new(Size {
            height: 4,
            width: 4,
        });
        let board_with_figure = board.replacing_figure_at_xy(0, 0, Some(FigureType::I));
        assert!(board_with_figure.contains(Point { x: 0, y: 0 }));
    }
}
