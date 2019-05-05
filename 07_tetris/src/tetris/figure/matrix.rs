#[derive(Debug, Clone, PartialEq)]
pub struct Matrix {
    pub data: Vec<Vec<u8>>,
}

impl Matrix {
    pub fn rotated(&self) -> Matrix {
        let mut data = vec![];
        let matrix_length = self.data.len();
        for i in 0..matrix_length {
            let mut vec = vec![];
            for j in 0..matrix_length {
                vec.push(self.data[(matrix_length - 1) - j][i]);
            }
            data.push(vec);
        }
        return Matrix { data };
    }

    pub fn at_xy(&self, x: usize, y: usize) -> u8 {
        return self.data[y][x];
    }
}
