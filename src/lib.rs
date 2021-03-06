//! A simple library to visualize 2D matrixes in rust.
//!
//! - Supports 256 terminal colours using AnsiTerm
//! - Multiple unicode box character sets supported (plain, retro, thin, rounded, thick, double)
//!
//! #Example use cases:
//! [chess-rs: a chess game](https://github.com/pierrechevalier83/chess-rs)
//!
//! [2048-rs: a game of 2048](https://github.com/pierrechevalier83/2048-rs)
//!
//! [palette-rs: a 256 colors palette](https://github.com/pierrechevalier83/palette-rs)
//!
//! #Example: visualising a chess board
//!
//! ```
//! extern crate matrix_display;
//! use matrix_display::*;
//!
//! fn main() {
//!     let format = Format::new(7, 3);
//!     let board = vec!['♜', '♞', '♝', '♛', '♚', '♝', '♞', '♜',
//!                      '♟', '♟', '♟', '♟', '♟', '♟', '♟', '♟',
//!                      ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
//!                      ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
//!                      ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
//!                      ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
//!                      '♖', '♘', '♗', '♕', '♔', '♗', '♘', '♖',
//!                      '♙', '♙', '♙', '♙', '♙', '♙', '♙', '♙']
//!         .iter()
//!         .enumerate()
//!         .map(|(i, x)| {
//!             let ansi_fg = 33;
//!             let mut ansi_bg = 0;
//!             if i % 2 + (i / 8) % 2 == 1 {
//!                 ansi_bg = 7;
//!             }
//!             cell::Cell::new(x.clone(), ansi_fg, ansi_bg)
//!             })
//!         .collect::<Vec<_>>();
//!     let mut data = matrix::Matrix::new(8, board);
//!     let mut display = MatrixDisplay::new(&format, &mut data);
//!     display.cell_at_cursor_position((13, 6)).color.bg = 10;
//!     display.print(&mut std::io::stdout(), &style::BordersStyle::None);
//! }
//! ```

pub mod cell;
pub mod matrix;
mod pad;
pub mod style;

use crate::cell::AnsiColor;
use crate::cell::Cell;
use crate::matrix::position::Position;
use crate::matrix::Matrix;
use crate::pad::horizontal_pad;
use crate::pad::Pad;
use crate::style::BordersStyle;

extern crate ansi_term;

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

/// Format of a cell in a matrix
///
/// Decide of the cell width and the cell height.
/// The matrix will pad its cells according to a Format.
///
/// Example:
/// `let format = matrix_display::Format::new(7,3)`
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
    use super::Format;
    use super::MatrixDisplay;
    use crate::cell::AnsiColor;
    use crate::cell::Cell;
    use crate::matrix::Matrix;
    #[test]
    fn width() {
        let f = Format::new(5, 7);
        let mut m = Matrix::new(
            3,
            (0..24)
                .map(|_| Cell::new(' ', AnsiColor::default().fg, AnsiColor::default().bg))
                .collect::<Vec<_>>(),
        );
        let d = MatrixDisplay::new(&f, &mut m);
        assert_eq!(3 * 5, d.width());
    }
    #[test]
    fn height() {
        let f = Format::new(5, 7);
        let mut m = Matrix::new(
            3,
            (0..24)
                .map(|_| Cell::new(' ', AnsiColor::default().fg, AnsiColor::default().bg))
                .collect::<Vec<_>>(),
        );
        let d = MatrixDisplay::new(&f, &mut m);
        assert_eq!(8 * 7, d.height());
    }
}

struct CellDisplay<'a> {
    borders: &'a BordersStyle,
}

impl<'a> CellDisplay<'a> {
    fn with_borders(b: &'a BordersStyle) -> CellDisplay {
        CellDisplay { borders: b }
    }
    fn top_cell(&self, pos: &Position, cell_width: usize) -> Vec<ansi_term::ANSIString<'static>> {
        if pos.top() {
            self.cell(
                self.borders.top_left_corner(),
                self.borders.top_intersection(),
                self.borders.top_right_corner(),
                self.borders.horizontal_border(),
                "",
                pos,
                cell_width,
                &AnsiColor::default(),
            )
        } else {
            self.cell(
                self.borders.left_intersection(),
                self.borders.intersection(),
                self.borders.right_intersection(),
                self.borders.horizontal_border(),
                "",
                pos,
                cell_width,
                &AnsiColor::default(),
            )
        }
    }
    fn bottom_cell(
        &self,
        pos: &Position,
        cell_width: usize,
    ) -> Vec<ansi_term::ANSIString<'static>> {
        if pos.bottom() {
            self.cell(
                self.borders.bottom_left_corner(),
                self.borders.bottom_intersection(),
                self.borders.bottom_right_corner(),
                self.borders.horizontal_border(),
                "",
                pos,
                cell_width,
                &AnsiColor::default(),
            )
        } else {
            Vec::new()
        }
    }
    fn padding_cell(
        &self,
        pos: &Position,
        cell_width: usize,
        color: &AnsiColor,
    ) -> Vec<ansi_term::ANSIString<'static>> {
        self.value_cell(pos, cell_width, " ", color)
    }
    fn value_cell(
        &self,
        pos: &Position,
        cell_width: usize,
        content: &str,
        color: &AnsiColor,
    ) -> Vec<ansi_term::ANSIString<'static>> {
        self.cell(
            self.borders.vertical_border(),
            self.borders.vertical_border(),
            self.borders.vertical_border(),
            ' ',
            content,
            pos,
            cell_width,
            color,
        )
    }
    fn cell(
        &self,
        left: char,
        middle: char,
        right: char,
        fill: char,
        content: &str,
        pos: &Position,
        width: usize,
        color: &AnsiColor,
    ) -> Vec<ansi_term::ANSIString<'static>> {
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
            if self.borders != &BordersStyle::None || content != "" {
                right_border += "\r\n";
            }
        }
        let plain = AnsiColor::default();

        vec![
            Fixed(plain.fg).on(Fixed(plain.bg)).paint(left_border),
            Fixed(color.fg).on(Fixed(color.bg)).paint(inside),
            Fixed(plain.fg).on(Fixed(plain.bg)).paint(right_border),
        ]
    }
}

fn cursor_to_index(x: usize, cell_dim: usize, n_cells: usize) -> usize {
    (std::cmp::min(x, n_cells * cell_dim) - 1) / cell_dim
}

/// Stores a matrix of data and offers a way to pretty print it
///
/// #Example: visualising a 256 colors palette:
/// ```
/// use matrix_display::*;
/// let format = Format::new(5,1);
/// let board = (0..256)
///        .map(|x| cell::Cell::new(x, 0, x as u8))
///     .collect::<Vec<_>>();
/// let mut data = matrix::Matrix::new(8, board);
/// let display = MatrixDisplay::new(&format, &mut data);
/// display.print(&mut std::io::stdout(), &style::BordersStyle::Light);
/// ```
pub struct MatrixDisplay<'a, T>
where
    T: Clone,
    T: ToString + 'a,
{
    fmt: &'a Format,
    mat: &'a mut Matrix<Cell<T>>,
}
impl<'a, T> MatrixDisplay<'a, T>
where
    T: Clone,
    T: ToString + 'a,
{
    /// Construct a matrix display
    ///
    /// f: the format of a cell (width, height)
    /// m: a reference to the data (&Matrix<Cell>)
    pub fn new(f: &'a Format, m: &'a mut Matrix<Cell<T>>) -> MatrixDisplay<'a, T> {
        MatrixDisplay { fmt: f, mat: m }
    }
    fn n_rows(&self) -> usize {
        self.mat.n_rows()
    }
    fn n_cols(&self) -> usize {
        self.mat.n_cols()
    }
    /// The matrix's width in number of characters
    pub fn width(&self) -> usize {
        self.n_cols() * self.fmt.cell_w
    }
    /// The matrix's height in number of characters
    pub fn height(&self) -> usize {
        self.n_rows() * self.fmt.cell_h
    }
    fn print_top(
        &self,
        borders: &BordersStyle,
        row: &[(Cell<T>, Position)],
    ) -> Vec<ansi_term::ANSIString<'static>> {
        row.iter()
            .flat_map(|&(_, ref pos)| {
                CellDisplay::with_borders(borders).top_cell(pos, self.fmt.cell_w)
            })
            .collect::<Vec<_>>()
    }
    fn print_vertical_pad(
        &self,
        borders: &BordersStyle,
        row: &[(Cell<T>, Position)],
        pad: usize,
    ) -> Vec<ansi_term::ANSIString<'static>> {
        row.iter()
            .cycle()
            .take(pad * row.len())
            .flat_map(|&(ref cell, ref pos)| {
                CellDisplay::with_borders(borders).padding_cell(pos, self.fmt.cell_w, &cell.color)
            })
            .collect::<Vec<_>>()
    }
    fn print_value_row(
        &self,
        borders: &BordersStyle,
        row: &[(Cell<T>, Position)],
    ) -> Vec<ansi_term::ANSIString<'static>> {
        row.iter()
            .flat_map(|&(ref cell, ref pos)| {
                CellDisplay::with_borders(borders).value_cell(
                    pos,
                    self.fmt.cell_w,
                    &cell.clone().value.to_string(),
                    &cell.color,
                )
            })
            .collect::<Vec<_>>()
    }
    fn print_bottom(
        &self,
        borders: &BordersStyle,
        row: &[(Cell<T>, Position)],
    ) -> Vec<ansi_term::ANSIString<'static>> {
        row.iter()
            .flat_map(|&(_, ref pos)| {
                CellDisplay::with_borders(borders).bottom_cell(pos, self.fmt.cell_w)
            })
            .collect::<Vec<_>>()
    }
    /// Render a matrix into a Vec<ANSIString>.
    ///
    /// Pick a BorderStyle, an output that implements the Write trait and
    /// you're good to go!
    /// This approach allows the user to customize how to display the matrix.
    pub fn render(&self, borders: &BordersStyle) -> Vec<ansi_term::ANSIString<'static>> {
        let vertical_pad = Pad::new(self.fmt.cell_h, 1);
        self.mat
            .enumerate_cells()
            .chunks(self.n_cols())
            .into_iter()
            .flat_map(|row| {
                let mut all_strings = self.print_top(borders, row);
                all_strings.append(&mut self.print_vertical_pad(borders, row, vertical_pad.before));
                all_strings.append(&mut self.print_value_row(borders, row));
                all_strings.append(&mut self.print_vertical_pad(borders, row, vertical_pad.after));
                all_strings.append(&mut self.print_bottom(borders, row));
                all_strings
                //    row
            })
            .collect::<Vec<_>>()
    }
    /// Print a matrix. This is the most important method of this library
    ///
    /// Pick a BorderStyle, an output that implements the Write trait and
    /// you're good to go!
    pub fn print<Out: Write>(&self, out: &mut Out, borders: &BordersStyle) {
        write!(out, "{}", ansi_term::ANSIStrings(&*self.render(borders))).unwrap();
    }
    /// Takes a cursor position in (usize, usize) and returns the coordinates of the cell under the cursor
    pub fn coordinates_at_cursor_position(&self, (x, y): (usize, usize)) -> (usize, usize) {
        let col = cursor_to_index(x, self.fmt.cell_w, self.mat.n_cols());
        let row = cursor_to_index(y as usize, self.fmt.cell_h, self.mat.n_rows());
        (col, row)
    }
    /// Takes a cursor position in characters (x, y) and returns a mutable reference to the corresponding cell
    ///
    /// This is used to modify a cell that was clicked
    pub fn cell_at_cursor_position(&mut self, cursor: (usize, usize)) -> &mut Cell<T> {
        let coord = self.coordinates_at_cursor_position(cursor);
        self.mat.at(coord)
    }
}
