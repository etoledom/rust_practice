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

    pub fn height(&self) -> u32 {
        return self.data.len() as u32;
    }

    pub fn width(&self) -> u32 {
        if let Some(line) = self.data.first() {
            return line.len() as u32;
        }
        return 0;
    }

    pub fn at_xy(&self, x: u32, y: u32) -> &T {
        return &self.data[y as usize][x as usize];
    }

    pub fn replacing_at_xy(&self, x: u32, y: u32, element: T) -> Matrix<T>
    where
        T: Clone,
    {
        let mut data = vec![];
        let matrix_length = self.data.len();
        for _y in 0..matrix_length {
            let mut vec = vec![];
            for _x in 0..matrix_length {
                if _x as u32 == x && _y as u32 == y {
                    vec.push(element.clone());
                }
                vec.push(self.data[_y][_x].clone());
            }
            data.push(vec);
        }
        return Matrix { data };
    }
}
