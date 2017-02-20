extern crate matrix_display;
use matrix_display::*;

fn main() {
    let format = Format::new(5, 1);
    let board = (0..256)
        .map(|x| cell::Cell::new(x, 0, x as u8))
        .collect::<Vec<_>>();
    let data = matrix::Matrix::new(8, board);
    let display = MatrixDisplay::new(format, data);
    display.print(&mut std::io::stdout(), &style::BordersStyle::Thin);
}
