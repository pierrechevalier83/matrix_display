#[cfg(test)]
mod cell_tests {
    use super::Cell;
    #[test]
    fn constructor() {
        let c = Cell::new('F', 42);
        assert_eq!(c.value, 'F');
        assert_eq!(c.ansi_code, 42);
    }
    fn clone_and_partial_eq() {
        let c = Cell::new('F', 42);
        let d = c.clone();
        assert_eq!(c, d);
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Cell {
    pub value: char,
    pub ansi_code: u8,
}
impl Cell {
    pub fn new(val: char, ansi: u8) -> Cell {
        Cell {
            value: val,
            ansi_code: ansi,
        }
    }
}

#[cfg(test)]
mod matrix_tests {
    use super::Cell;
    use super::Matrix;
    #[test]
    fn constructor() {
        let n = 4;
        let v = (0..24).map(|x| Cell::new(' ', x)).collect::<Vec<_>>();
        let m = Matrix::new(n, v.clone());
        assert_eq!(m.n_cols, n);
        assert_eq!(m.cells, v);
    }
}

pub struct Matrix {
    n_cols: usize,
    cells: Vec<Cell>,
}
impl Matrix {
    fn new(n_cols: usize, cells: Vec<Cell>) -> Matrix {
        Matrix {
            n_cols: n_cols,
            cells: cells,
        }
    }
    pub fn n_rows(&self) -> usize {
        self.cells.len() / self.n_cols
    }
    pub fn n_cols(&self) -> usize {
        self.n_cols
    }
}

#[cfg(test)]
mod format_tests {
    use super::Format;
    #[test]
    fn constructor() {
        let f = Format::new(3, 4);
        assert_eq!(f.cell_w, 3);
        assert_eq!(f.cell_h, 4);
    }
    #[test]
    fn default() {
        let f = Format::default();
        assert_eq!(f.cell_w, 1);
        assert_eq!(f.cell_h, 1);
    }
}

pub struct Format {
    pub cell_w: usize,
    pub cell_h: usize,
}
impl Format {
    pub fn new(w: usize, h: usize) -> Format {
        Format {
            cell_w: w,
            cell_h: h,
        }
    }
}
impl Default for Format {
    fn default() -> Format {
        Format::new(1, 1)
    }
}


#[cfg(test)]
mod matrix_display_tests {
    use super::Cell;
    use super::Matrix;
    use super::Format;
    use super::MatrixDisplay;
    #[test]
    fn width() {
        let f = Format::new(5, 7);
        let m = Matrix::new(3, (0..24).map(|x| Cell::new(' ', x)).collect::<Vec<_>>());
        let d = MatrixDisplay::new(f, m);
        assert_eq!(3 * 5, d.width());
    }
    fn height() {
        let f = Format::new(5, 7);
        let m = Matrix::new(3, (0..24).map(|x| Cell::new(' ', x)).collect::<Vec<_>>());
        let d = MatrixDisplay::new(f, m);
        assert_eq!(8 * 7, d.height());
    }
}

pub struct MatrixDisplay {
    fmt: Format,
    mat: Matrix,
}
impl MatrixDisplay {
    fn new(f: Format, m: Matrix) -> MatrixDisplay {
        MatrixDisplay { fmt: f, mat: m }
    }
    fn n_rows(&self) -> usize {
        self.mat.n_rows()
    }
    fn n_cols(&self) -> usize {
        self.mat.n_cols()
    }
    pub fn width(&self) -> usize {
        self.n_cols() * self.fmt.cell_w
    }
    pub fn height(&self) -> usize {
        self.n_rows() * self.fmt.cell_h
    }
}
