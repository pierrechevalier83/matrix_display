pub mod cell;
pub mod matrix;

use cell::AnsiColor;
use cell::Cell;
use matrix::position::Position;
use matrix::Matrix;

extern crate unicode_segmentation;
extern crate ansi_term;

use unicode_segmentation::UnicodeSegmentation;
use ansi_term::Colour::Fixed;
use std::io::Write;

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
    #[test]
    pub fn no_pad_even() {
        let pad = Pad::new(4, 4);
        assert_eq!(0, pad.before);
        assert_eq!(0, pad.after);
    }
    #[test]
    pub fn no_pad_odd() {
        let pad = Pad::new(1, 1);
        assert_eq!(0, pad.before);
        assert_eq!(0, pad.after);
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
    use cell::Cell;
    use cell::AnsiColor;
    use matrix::Matrix;
    use super::Format;
    use super::MatrixDisplay;
    #[test]
    fn width() {
        let f = Format::new(5, 7);
        let m = Matrix::new(3,
                            (0..24)
                                .map(|_| {
                                    Cell::new(' ', AnsiColor::default().fg, AnsiColor::default().bg)
                                })
                                .collect::<Vec<_>>());
        let d = MatrixDisplay::new(f, m);
        assert_eq!(3 * 5, d.width());
    }
    #[test]
    fn height() {
        let f = Format::new(5, 7);
        let m = Matrix::new(3,
                            (0..24)
                                .map(|_| {
                                    Cell::new(' ', AnsiColor::default().fg, AnsiColor::default().bg)
                                })
                                .collect::<Vec<_>>());
        let d = MatrixDisplay::new(f, m);
        assert_eq!(8 * 7, d.height());
    }
}

// Functions that give the characters surrounding a cell TLRB
pub enum BoxStyle {
    Plain, /* . .
            *
            * . .
            *
            * */
    Retro, /* +-+-+
            * |.|.|
            * +-+-+
            * |.|.|
            * +-+-+
            * */
    Thin, /* ┌─┬─┐
           * │.│.│
           * ├─┼─┤
           * │.│.│
           * └─┴─┘
           * */
    Rounded, /* ╭─┬─╮
              * │.│.│
              * ├─┼─┤
              * │.│.│
              * ╰─┴─╯
              * */
    Thick, /* ┏━┳━┓
            * ┃.┃.┃
            * ┣━╋━┫
            * ┃.┃.┃
            * ┗━┻━┛
            * */
    Double, /* ╔═╦═╗
             * ║.║.║
             * ╠═╬═╣
             * ║.║.║
             * ╚═╩═╝
             * */
}

fn horizontal_pad(width: usize, s: &str, c: char) -> String {
    let pad = Pad::new(width, s.graphemes(true).count());
    std::iter::repeat(c).take(pad.before).collect::<String>() + s +
    &std::iter::repeat(c).take(pad.after).collect::<String>()
}


impl BoxStyle {
    fn top_left_corner(&self) -> char {
        match *self {
            BoxStyle::Plain => ' ',
            BoxStyle::Retro => '+',
            BoxStyle::Thin => '┌',
            BoxStyle::Rounded => '╭',
            BoxStyle::Thick => '┏',
            BoxStyle::Double => '╔',
        }
    }
    fn top_right_corner(&self) -> char {
        match *self {
            BoxStyle::Plain => ' ',
            BoxStyle::Retro => '+',
            BoxStyle::Thin => '┐',
            BoxStyle::Rounded => '╮',
            BoxStyle::Thick => '┓',
            BoxStyle::Double => '╗',
        }
    }
    fn bottom_left_corner(&self) -> char {
        match *self {
            BoxStyle::Plain => ' ',
            BoxStyle::Retro => '+',
            BoxStyle::Thin => '└',
            BoxStyle::Rounded => '╰',
            BoxStyle::Thick => '┗',
            BoxStyle::Double => '╚',
        }
    }
    fn bottom_right_corner(&self) -> char {
        match *self {
            BoxStyle::Plain => ' ',
            BoxStyle::Retro => '+',
            BoxStyle::Thin => '┘',
            BoxStyle::Rounded => '╯',
            BoxStyle::Thick => '┛',
            BoxStyle::Double => '╝',
        }
    }
    fn horizontal_border(&self) -> char {
        match *self {
            BoxStyle::Plain => ' ',
            BoxStyle::Retro => '-',
            BoxStyle::Thin => '─',
            BoxStyle::Rounded => '─',
            BoxStyle::Thick => '━',
            BoxStyle::Double => '═',
        }
    }
    fn top_intersection(&self) -> char {
        match *self {
            BoxStyle::Plain => ' ',
            BoxStyle::Retro => '+',
            BoxStyle::Thin => '┬',
            BoxStyle::Rounded => '┬',
            BoxStyle::Thick => '┳',
            BoxStyle::Double => '╦',
        }
    }
    fn left_intersection(&self) -> char {
        match *self {
            BoxStyle::Plain => ' ',
            BoxStyle::Retro => '+',
            BoxStyle::Thin => '├',
            BoxStyle::Rounded => '├',
            BoxStyle::Thick => '┣',
            BoxStyle::Double => '╠',
        }
    }
    fn right_intersection(&self) -> char {
        match *self {
            BoxStyle::Plain => ' ',
            BoxStyle::Retro => '+',
            BoxStyle::Thin => '┤',
            BoxStyle::Rounded => '┤',
            BoxStyle::Thick => '┫',
            BoxStyle::Double => '╣',
        }
    }
    fn bottom_intersection(&self) -> char {
        match *self {
            BoxStyle::Plain => ' ',
            BoxStyle::Retro => '+',
            BoxStyle::Thin => '┴',
            BoxStyle::Rounded => '┴',
            BoxStyle::Thick => '┻',
            BoxStyle::Double => '╩',
        }
    }
    fn intersection(&self) -> char {
        match *self {
            BoxStyle::Plain => ' ',
            BoxStyle::Retro => '+',
            BoxStyle::Thin => '┼',
            BoxStyle::Rounded => '┼',
            BoxStyle::Thick => '╋',
            BoxStyle::Double => '╬',
        }
    }
    fn vertical_border(&self) -> char {
        match *self {
            BoxStyle::Plain => ' ',
            BoxStyle::Retro => '|',
            BoxStyle::Thin => '│',
            BoxStyle::Rounded => '│',
            BoxStyle::Thick => '┃',
            BoxStyle::Double => '║',
        }
    }
    fn top_cell(&self, pos: &Position, cell_width: usize) -> String {
        if pos.top() {
            self.cell(self.top_left_corner(),
                      self.top_intersection(),
                      self.top_right_corner(),
                      self.horizontal_border(),
                      "",
                      pos,
                      cell_width,
                      &AnsiColor::default())
        } else {
            self.cell(self.left_intersection(),
                      self.intersection(),
                      self.right_intersection(),
                      self.horizontal_border(),
                      "",
                      pos,
                      cell_width,
                      &AnsiColor::default())
        }
    }
    fn bottom_cell(&self, pos: &Position, cell_width: usize) -> String {
        if pos.bottom() {
            self.cell(self.bottom_left_corner(),
                      self.bottom_intersection(),
                      self.bottom_right_corner(),
                      self.horizontal_border(),
                      "",
                      pos,
                      cell_width,
                      &AnsiColor::default())
        } else {
            String::new()
        }
    }
    fn padding_cell(&self, pos: &Position, cell_width: usize, color: &AnsiColor) -> String {
        self.value_cell(pos, cell_width, "", color)
    }
    fn value_cell(&self,
                  pos: &Position,
                  cell_width: usize,
                  content: &str,
                  color: &AnsiColor)
                  -> String {
        self.cell(self.vertical_border(),
                  self.vertical_border(),
                  self.vertical_border(),
                  ' ',
                  content,
                  pos,
                  cell_width,
                  color)
    }
    fn cell(&self,
            left: char,
            middle: char,
            right: char,
            fill: char,
            content: &str,
            pos: &Position,
            width: usize,
            color: &AnsiColor)
            -> String {
        let mut left_border = String::new();
        if pos.left() {
            left_border += &left.to_string();
        } else {
            left_border += &middle.to_string();
        }
        let inside = horizontal_pad(width, content, fill);

        let mut right_border = String::new();
        if pos.right() {
            right_border += &right.to_string();
            right_border += "\n";
        }
        let plain = AnsiColor::default();

        Fixed(plain.fg).on(Fixed(plain.bg)).paint(left_border).to_string() +
        &Fixed(color.fg).on(Fixed(color.bg)).paint(inside).to_string() +
        &Fixed(plain.fg).on(Fixed(plain.bg)).paint(right_border).to_string()
    }
}

pub struct MatrixDisplay<T>
    where T: Clone,
          T: ToString
{
    fmt: Format,
    mat: Matrix<Cell<T>>,
}
impl<T> MatrixDisplay<T>
    where T: Clone,
          T: ToString
{
    pub fn new(f: Format, m: Matrix<Cell<T>>) -> MatrixDisplay<T> {
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
    fn print_top<Out: Write>(&self, out: &mut Out, style: &BoxStyle, row: &[(Cell<T>, Position)]) {
        for &(_, ref pos) in row {
            write!(out, "{}", style.top_cell(pos, self.fmt.cell_w)).unwrap();
        }
    }
    fn print_vertical_pad<Out: Write>(&self,
                                      out: &mut Out,
                                      style: &BoxStyle,
                                      row: &[(Cell<T>, Position)],
                                      pad: usize) {
        for _ in 0..pad {
            for &(ref cell, ref pos) in row {
                write!(out,
                       "{}",
                       style.padding_cell(pos, self.fmt.cell_w, &cell.color))
                    .unwrap();
            }
        }
    }
    fn print_value_row<Out: Write>(&self,
                                   out: &mut Out,
                                   style: &BoxStyle,
                                   row: &[(Cell<T>, Position)]) {
        for &(ref cell, ref pos) in row {
            write!(out,
                   "{}",
                   style.value_cell(pos,
                                    self.fmt.cell_w,
                                    &cell.clone().value.to_string(),
                                    &cell.color))
                .unwrap();
        }
    }
    fn print_bottom<Out: Write>(&self,
                                out: &mut Out,
                                style: &BoxStyle,
                                row: &[(Cell<T>, Position)]) {
        for &(_, ref pos) in row {
            write!(out, "{}", style.bottom_cell(pos, self.fmt.cell_w)).unwrap();
        }
    }
    pub fn print<Out: Write>(&self, out: &mut Out, style: &BoxStyle) {
        let vertical_pad = Pad::new(self.fmt.cell_h, 1);
        self.mat
            .enumerate_cells()
            .chunks(self.n_cols())
            .into_iter()
            .flat_map(|row| {
                self.print_top(out, style, row);
                self.print_vertical_pad(out, style, row, vertical_pad.before);
                self.print_value_row(out, style, row);
                self.print_vertical_pad(out, style, row, vertical_pad.after);
                self.print_bottom(out, style, row);
                row
            })
            .collect::<Vec<_>>();
    }
}
