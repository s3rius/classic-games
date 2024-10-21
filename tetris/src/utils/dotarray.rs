#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DotArray<T> {
    pub dots: Vec<(usize, usize, T)>,
    dimensions: (usize, usize),
    max: usize,
}

impl<T: Default + PartialEq> From<Vec<Vec<T>>> for DotArray<T> {
    fn from(value: Vec<Vec<T>>) -> Self {
        let default = T::default();
        let mut dots = vec![];
        let max_y = value.len() - 1;
        let max_x = value[0].len() - 1;
        for (y, row) in value.into_iter().enumerate() {
            for (x, value) in row.into_iter().enumerate() {
                if value != default {
                    dots.push((y, x, value));
                }
            }
        }
        Self {
            dots,
            dimensions: (max_y, max_x),
            max: max_y.max(max_x),
        }
    }
}

impl<T> From<Vec<Vec<Option<T>>>> for DotArray<T> {
    fn from(value: Vec<Vec<Option<T>>>) -> Self {
        let mut dots = vec![];
        let max_y = value.len() - 1;
        let max_x = value[0].len() - 1;
        for (index, row) in value.into_iter().enumerate() {
            for (j, value) in row.into_iter().enumerate() {
                if let Some(inner) = value {
                    dots.push((index, j, inner));
                }
            }
        }
        Self {
            dots,
            dimensions: (max_y, max_x),
            max: max_y.max(max_x),
        }
    }
}

impl<T> DotArray<T> {
    pub fn rotate_left(&mut self) {
        let max = self.max;
        self.dimensions = (self.dimensions.1, self.dimensions.0);
        for (y, x, _) in self.dots.iter_mut() {
            std::mem::swap(x, y);
            *y = max - *y;
        }
    }
    pub fn rotate_right(&mut self) {
        let max = self.max;
        self.dimensions = (self.dimensions.1, self.dimensions.0);
        for (y, x, _) in self.dots.iter_mut() {
            std::mem::swap(x, y);
            *x = max - *x;
        }
    }
}
