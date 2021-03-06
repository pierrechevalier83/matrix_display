//! Provides `Matrix` which stores a matrix of arbitrary data
//!
//! Data is stored as a vector in row major order and a usize representing the number of columns
//! Data can be accessed for reading through the `enumerate_cells` function
//! That function returns a vector of a clone of each cell and its `Position`
pub mod position;
use self::position::Position;

#[cfg(test)]
mod matrix_tests {
    use super::position::Position;
    use super::Matrix;
    use crate::cell::AnsiColor;
    use crate::cell::Cell;
    #[test]
    fn constructor() {
        let n = 4;
        let v = (0..24)
            .map(|x| Cell::new(' ', x, AnsiColor::default().bg))
            .collect::<Vec<_>>();
        let m = Matrix::new(n, v.clone());
        assert_eq!(m.n_cols, n);
        assert_eq!(m.cells, v);
    }
    #[test]
    fn from_index_with_square_matrix() {
        // 0,  1,  2,
        // 3,  4,  5,
        // 6,  7,  8,
        //
        let n = 3;
        let v = (0..11)
            .map(|_| Cell::new(' ', AnsiColor::default().fg, AnsiColor::default().bg))
            .collect::<Vec<_>>();
        let m = Matrix::new(n, v.clone());
        assert_eq!(Position::TopLeft, m.from_index(0));
        assert_eq!(Position::Top, m.from_index(1));
        assert_eq!(Position::TopRight, m.from_index(2));
        assert_eq!(Position::Left, m.from_index(3));
        assert_eq!(Position::Middle, m.from_index(4));
        assert_eq!(Position::Right, m.from_index(5));
        assert_eq!(Position::BottomLeft, m.from_index(6));
        assert_eq!(Position::Bottom, m.from_index(7));
        assert_eq!(Position::BottomRight, m.from_index(8));
    }
    #[test]
    fn from_index_less_cols_than_rows() {
        // 0,  1,  2,
        // 3,  4,  5,
        // 6,  7,  8,
        // 9, 10, 11
        //
        let n = 3;
        let v = (0..12)
            .map(|_| Cell::new(' ', AnsiColor::default().fg, AnsiColor::default().bg))
            .collect::<Vec<_>>();
        let m = Matrix::new(n, v.clone());
        assert_eq!(Position::TopLeft, m.from_index(0));
        assert_eq!(Position::Top, m.from_index(1));
        assert_eq!(Position::TopRight, m.from_index(2));
        assert_eq!(Position::Left, m.from_index(3));
        assert_eq!(Position::Middle, m.from_index(4));
        assert_eq!(Position::Right, m.from_index(5));
        assert_eq!(Position::Left, m.from_index(6));
        assert_eq!(Position::Middle, m.from_index(7));
        assert_eq!(Position::Right, m.from_index(8));
        assert_eq!(Position::BottomLeft, m.from_index(9));
        assert_eq!(Position::Bottom, m.from_index(10));
        assert_eq!(Position::BottomRight, m.from_index(11));
    }
    #[test]
    fn from_index_less_rows_than_cols() {
        // 0, 1,  2, 3,
        // 4, 5,  6, 7,
        // 8, 9, 10, 11
        //
        let n = 4;
        let v = (0..12)
            .map(|_| Cell::new(' ', AnsiColor::default().fg, AnsiColor::default().bg))
            .collect::<Vec<_>>();
        let m = Matrix::new(n, v.clone());
        assert_eq!(Position::TopLeft, m.from_index(0));
        assert_eq!(Position::Top, m.from_index(1));
        assert_eq!(Position::Top, m.from_index(2));
        assert_eq!(Position::TopRight, m.from_index(3));
        assert_eq!(Position::Left, m.from_index(4));
        assert_eq!(Position::Middle, m.from_index(5));
        assert_eq!(Position::Middle, m.from_index(6));
        assert_eq!(Position::Right, m.from_index(7));
        assert_eq!(Position::BottomLeft, m.from_index(8));
        assert_eq!(Position::Bottom, m.from_index(9));
        assert_eq!(Position::Bottom, m.from_index(10));
        assert_eq!(Position::BottomRight, m.from_index(11));
    }
}

/// Stores a matrix of arbitrary data
///
/// Gives mutable access to cell at given position
/// Gives owned clone of data with position of each cell within the matrix
#[derive(Clone)]
pub struct Matrix<T>
where
    T: Clone,
{
    n_cols: usize,
    cells: Vec<T>,
}
impl<T> Matrix<T>
where
    T: Clone,
{
    /// Construct a matrix with the number of columns and the data represented as a row-major ordered `Vec`
    pub fn new(n_cols: usize, cells: Vec<T>) -> Matrix<T> {
        Matrix {
            n_cols: n_cols,
            cells: cells,
        }
    }
    /// Number of rows in the matrix
    pub fn n_rows(&self) -> usize {
        self.cells.len() / self.n_cols()
    }
    /// Number of columns in the matrix
    pub fn n_cols(&self) -> usize {
        self.n_cols
    }
    /// Mutably access element at given position (row index, column index)
    pub fn at(&mut self, (x, y): (usize, usize)) -> &mut T {
        &mut self.cells[x + self.n_cols * y]
    }
    fn is_top(&self, index: usize) -> bool {
        index < self.n_cols()
    }
    fn is_left(&self, index: usize) -> bool {
        index % self.n_cols() == 0
    }
    fn is_right(&self, index: usize) -> bool {
        index % self.n_cols() == self.n_cols() - 1
    }
    fn is_bottom(&self, index: usize) -> bool {
        index >= (self.n_rows() - 1) * self.n_cols()
    }
    fn from_index(&self, index: usize) -> position::Position {
        if self.is_top(index) && self.is_left(index) {
            Position::TopLeft
        } else if self.is_top(index) && self.is_right(index) {
            Position::TopRight
        } else if self.is_bottom(index) && self.is_left(index) {
            Position::BottomLeft
        } else if self.is_bottom(index) && self.is_right(index) {
            Position::BottomRight
        } else if self.is_top(index) {
            Position::Top
        } else if self.is_left(index) {
            Position::Left
        } else if self.is_right(index) {
            Position::Right
        } else if self.is_bottom(index) {
            Position::Bottom
        } else {
            Position::Middle
        }
    }
    /// View data stored in the matrix
    ///
    /// The returned data is a clone of the data owned by the `Matrix` class and is owned by the caller
    /// The second element of the returned tuples indicates the position of that element within the matrix
    pub fn enumerate_cells(&self) -> Vec<(T, Position)> {
        self.cells
            .clone()
            .into_iter()
            .enumerate()
            .map(|(idx, cell)| (cell, self.from_index(idx)))
            .collect::<Vec<_>>()
    }
}
