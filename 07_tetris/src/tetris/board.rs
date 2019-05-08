use super::FigureType;
use super::Size;

pub struct Board {
    cells: Vec<Vec<Option<FigureType>>>,
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
        return Board { cells };
    }

    pub fn height(&self) -> u32 {
        return self.cells.len() as u32;
    }

    pub fn width(&self) -> u32 {
        if let Some(line) = self.cells.first() {
            return line.len() as u32;
        }
        return 0;
    }

    pub fn figure_at_xy(&self, x: u32, y: u32) -> &Option<FigureType> {
        return &self.cells[y as usize][x as usize];
    }

    pub fn replace_figure_at_xy(&mut self, x: u32, y: u32, figure_type: Option<FigureType>) {
        self.cells[y as usize][x as usize] = figure_type;
    }
}
