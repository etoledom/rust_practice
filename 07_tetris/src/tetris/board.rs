use super::figure::Matrix;
use super::FigureType;
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
        return self.matrix.at_xy(x, y);
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
}
