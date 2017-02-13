use std::io::Write;

#[cfg(test)]
mod cell_tests {
    use super::Cell;
    #[test]
    fn constructor() {
        let c = Cell::new('F', 42);
        assert_eq!(c.value, 'F');
        assert_eq!(c.ansi_code, 42);
    }
    #[test]
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

#[derive(Debug, PartialEq)]
pub enum Position {
    Top,
    Left,
    Right,
    Bottom,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Middle,
}

#[cfg(test)]
mod matrix_tests {
    use super::Cell;
    use super::Matrix;
    use super::Position;
    #[test]
    fn constructor() {
        let n = 4;
        let v = (0..24).map(|x| Cell::new(' ', x)).collect::<Vec<_>>();
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
        let v = (0..11).map(|x| Cell::new(' ', x)).collect::<Vec<_>>();
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
        let v = (0..12).map(|x| Cell::new(' ', x)).collect::<Vec<_>>();
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
        let v = (0..12).map(|x| Cell::new(' ', x)).collect::<Vec<_>>();
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

pub struct Matrix {
    n_cols: usize,
    cells: Vec<Cell>,
}
impl Matrix {
    pub fn new(n_cols: usize, cells: Vec<Cell>) -> Matrix {
        Matrix {
            n_cols: n_cols,
            cells: cells,
        }
    }
    pub fn n_rows(&self) -> usize {
        self.cells.len() / self.n_cols()
    }
    pub fn n_cols(&self) -> usize {
        self.n_cols
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
    pub fn from_index(&self, index: usize) -> Position {
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
    pub fn enumerate_cells(&mut self) -> Vec<(Cell, Position)> {
        self.cells
            .clone()
            .into_iter()
            .enumerate()
            .map(|(idx, cell)| (cell, self.from_index(idx)))
            .collect::<Vec<_>>()
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
mod pad_tests {
    use super::Pad;
    #[test]
    pub fn adds_up_to_correct_size() {
        let pad = Pad::new(12, 3);
        assert_eq!(12, pad.before + 3 + pad.after);
    }
    #[test]
    pub fn centered() {
        let pad = Pad::new(10, 2);
        assert_eq!(4, pad.before);
        assert_eq!(4, pad.after);
    }
    #[test]
    pub fn odd_total() {
        let pad = Pad::new(11, 3);
        assert_eq!(4, pad.before);
        assert_eq!(4, pad.after);
    }
}

pub struct Pad {
    pub before: usize,
    pub after: usize,
}
impl Pad {
    pub fn new(total: usize, content: usize) -> Pad {
        Pad {
            before: (total - content) / 2 + (total - content) % 2,
            after: (total - content) / 2,
        }
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
    #[test]
    fn height() {
        let f = Format::new(5, 7);
        let m = Matrix::new(3, (0..24).map(|x| Cell::new(' ', x)).collect::<Vec<_>>());
        let d = MatrixDisplay::new(f, m);
        assert_eq!(8 * 7, d.height());
    }
}

pub struct CharBox {
    top: Option<char>,
	left: Option<char>,
	right: Option<char>,
	bottom: Option<char>,
}
impl CharBox {
    fn new(t: Option<char>, l: Option<char>, r: Option<char>, b: Option<char>) -> CharBox {
	    CharBox{
		    top: t,
			left: l,
			right: r,
			bottom: b
		}
	}
}

// Functions that give the characters surrounding a cell TLRB
pub struct BoxStyle {
}
impl BoxStyle {
    pub fn plain(pos: &Position) -> CharBox {
	    match *pos {
		    Position::Right => CharBox::new(None,None,Some('\n'),None),
		    Position::TopRight => CharBox::new(None,None, Some('\n'),None),
		    Position::BottomRight => CharBox::new(None,None, Some('\n'),None),
		    _ => CharBox::new(None, None, None, None)
		}
	}
	pub fn unicode(pos: &Position) -> &'static str {
	    match *pos {
		    Position::Top => " ",
		    Position::Left => " ",
		    Position::Right => " ",
		    Position::Bottom => " ",
		    Position::TopLeft => " ",
		    Position::TopRight => " ",
		    Position::BottomLeft => " ",
		    Position::BottomRight => " ",
		    Position::Middle => " ",
		}
	}
}

pub struct MatrixDisplay {
    fmt: Format,
    mat: Matrix,
}
impl MatrixDisplay {
    pub fn new(f: Format, m: Matrix) -> MatrixDisplay {
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
    pub fn print<Out: Write>(&mut self, out: &mut Out) {
        let style = BoxStyle::plain;
		self.mat
            .enumerate_cells()
            .iter()
            .map(|&(ref cell, ref pos)| {
			    let around = style(pos);
				if let Some(c) = around.left {
                   write!(out, "{}", c).unwrap();
				}
                write!(out, "{}", cell.value).unwrap();
				if let Some(c) = around.right {
                   write!(out, "{}", c).unwrap();
				}
            })
            .collect::<Vec<_>>();
    }
}
