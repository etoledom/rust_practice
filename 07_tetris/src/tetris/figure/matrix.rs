#[derive(Debug, Clone, PartialEq)]
pub struct Matrix<T> {
    data: Vec<Vec<T>>,
}

impl<T> Matrix<T> {
    pub fn new(data: Vec<Vec<T>>) -> Matrix<T> {
        return Matrix { data };
    }

    pub fn rotated(&self) -> Matrix<T>
    where
        T: Clone,
    {
        let mut data = vec![];
        let matrix_length = self.data.len();
        for i in 0..matrix_length {
            let mut vec = vec![];
            for j in 0..matrix_length {
                let y_position = (matrix_length - 1) - j;
                let element = self.data[y_position][i].clone();
                vec.push(element);
            }
            data.push(vec);
        }
        return Matrix { data };
    }

    pub fn height(&self) -> usize {
        return self.data.len();
    }

    pub fn width(&self) -> usize {
        if let Some(line) = self.data.first() {
            return line.len();
        }
        return 0;
    }

    pub fn at_xy(&self, x: usize, y: usize) -> &T {
        return &self.data[y][x];
    }

    pub fn replacing_at_xy(&self, x: usize, y: usize, element: T) -> Matrix<T>
    where
        T: Clone,
    {
        let mut data = vec![];
        let matrix_length = self.data.len();
        for _y in 0..matrix_length {
            let mut vec = vec![];
            for _x in 0..matrix_length {
                if _x == x && _y == y {
                    vec.push(element.clone());
                }
                vec.push(self.data[_y][_x].clone());
            }
            data.push(vec);
        }
        return Matrix { data };
    }
}
