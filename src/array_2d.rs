use std::fmt::Display;

pub struct Array2D<T> {
    n_rows: usize,
    n_cols: usize,
    values: Vec<T>,
}

#[allow(dead_code)]
impl<T> Array2D<T> {
    pub fn from_string<F: Fn(char) -> T>(raw_input: &str, n_rows: usize, n_cols: usize, char_transformer: F) -> Self {
        let len: usize = raw_input.len();
        assert_eq!(n_rows * n_cols, len, "Given 2D array size was expected to be {n_rows}x{n_cols} but got {len}!");
        let mut values: Vec<T> = Vec::with_capacity(len);

        for line in raw_input.lines().peekable() {
            for c in line.chars() {
                values.push(char_transformer(c));
            }
        }

        Self { n_rows, n_cols, values }
    }

    fn is_within(&self, coordinate: Index2D) -> bool {
        coordinate.row < self.n_rows && coordinate.col < self.n_cols
    }

    fn get_by_point(&self, coordinate: Index2D) -> Option<&T> {
        if self.is_within(coordinate) {
            Some(&self.values[self.n_cols * coordinate.row + coordinate.col])
        } else {
            None
        }
    }

    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        self.get_by_point(Index2D{row, col})
    }
}

#[allow(dead_code)]
impl<T: Display> Array2D<T> {
    pub fn print(&self) {
        for row in 0..self.n_rows {
            for col in 0..self.n_cols {
                print!("{} ", self.get(row, col).unwrap());
            }
            println!();
        }
    }
}

#[derive(Copy, Clone)]
pub struct Index2D {
    row: usize,
    col: usize,
}

#[allow(dead_code)]
impl Index2D {
    pub fn row(&self) -> usize {
        self.row
    }

    pub fn col(&self) -> usize {
        self.col
    }
}

#[derive(Copy, Clone)]
pub struct Coordinate {
    x: isize,
    y: isize,
}

#[allow(dead_code)]
impl Coordinate {
    pub fn x(&self) -> isize {
        self.x
    }

    pub fn y(&self) -> isize {
        self.y
    }

    pub fn from_coordinates(x: isize, y: isize) -> Self {
        Coordinate {x, y}
    }

    pub fn change_x(&mut self, new_x: isize) {
        self.x = new_x;
    }

    pub fn increment_x(&mut self, increment: isize) {
        self.x += increment;
    }

    pub fn decrement_x(&mut self, decrement: isize) {
        self.x -= decrement;
    }

    pub fn change_y(&mut self, new_y: isize) {
        self.y = new_y;
    }

    pub fn increment_y(&mut self, increment: isize) {
        self.y += increment;
    }

    pub fn decrement_y(&mut self, decrement: isize) {
        self.y -= decrement;
    }
}