use super::matrix::Matrix;
use utilities::graphics::Color;

const I_COLOR: Color = Color {
    red: 108.0 / 255.0,
    green: 237.0 / 255.0,
    blue: 238.0 / 255.0,
    alpha: 1.0,
};

const J_COLOR: Color = Color {
    red: 0.0,
    green: 33.0 / 255.0,
    blue: 230.0 / 255.0,
    alpha: 1.0,
};

const L_COLOR: Color = Color {
    red: 229.0 / 255.0,
    green: 162.0 / 255.0,
    blue: 67.0 / 255.0,
    alpha: 1.0,
};

const O_COLOR: Color = Color {
    red: 241.0 / 255.0,
    green: 238.0 / 255.0,
    blue: 79.0 / 255.0,
    alpha: 1.0,
};

const Z_COLOR: Color = Color {
    red: 110.0 / 255.0,
    green: 235.0 / 255.0,
    blue: 71.0 / 255.0,
    alpha: 1.0,
};

const T_COLOR: Color = Color {
    red: 146.0 / 255.0,
    green: 45.0 / 255.0,
    blue: 231.0 / 255.0,
    alpha: 1.0,
};

const S_COLOR: Color = Color {
    red: 221.0 / 255.0,
    green: 47.0 / 255.0,
    blue: 23.0 / 255.0,
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
        return match self {
            FigureType::I => I_COLOR,
            FigureType::J => J_COLOR,
            FigureType::L => L_COLOR,
            FigureType::O => O_COLOR,
            FigureType::S => S_COLOR,
            FigureType::T => T_COLOR,
            FigureType::Z => Z_COLOR,
        };
    }

    pub fn initial_matrix(&self) -> Matrix {
        let vectors = match self {
            FigureType::I => self.draw_I(),
            FigureType::J => self.draw_J(),
            FigureType::L => self.draw_L(),
            FigureType::O => self.draw_O(),
            FigureType::S => self.draw_S(),
            FigureType::T => self.draw_T(),
            FigureType::Z => self.draw_Z(),
        };
        return Matrix { data: vectors };
    }

    fn draw_I(&self) -> Vec<Vec<u8>> {
        return vec![
            vec![0, 0, 0, 0], //
            vec![1, 1, 1, 1],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
        ];
    }

    fn draw_J(&self) -> Vec<Vec<u8>> {
        return vec![
            vec![1, 0, 0], //
            vec![1, 1, 1],
            vec![0, 0, 0],
        ];
    }

    fn draw_L(&self) -> Vec<Vec<u8>> {
        return vec![
            vec![0, 0, 1], //
            vec![1, 1, 1],
            vec![0, 0, 0],
        ];
    }

    fn draw_O(&self) -> Vec<Vec<u8>> {
        return vec![
            vec![1, 1], //
            vec![1, 1],
        ];
    }

    fn draw_S(&self) -> Vec<Vec<u8>> {
        return vec![
            vec![0, 1, 1], //
            vec![1, 1, 0],
            vec![0, 0, 0],
        ];
    }

    fn draw_T(&self) -> Vec<Vec<u8>> {
        return vec![
            vec![0, 1, 0], //
            vec![1, 1, 1],
            vec![0, 0, 0],
        ];
    }

    fn draw_Z(&self) -> Vec<Vec<u8>> {
        return vec![
            vec![1, 1, 0], //
            vec![0, 1, 1],
            vec![0, 0, 0],
        ];
    }
}
