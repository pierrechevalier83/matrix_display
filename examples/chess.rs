extern crate matrix_display;
use matrix_display::*;

fn main() {
    let format = Format::new(5, 3); //default();
    let board = vec!['♜', '♞', '♝', '♛', '♚', '♝', '♞', '♜',
	                 '♟', '♟', '♟', '♟', '♟', '♟', '♟', '♟',
					 ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                     ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
					 ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
					 ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
					 '♖', '♘', '♗', '♕', '♔', '♗', '♘', '♖',
					 '♙', '♙', '♙', '♙', '♙', '♙', '♙', '♙']
        .iter()
        .map(|x| Cell::new(x.clone(), 0))
        .collect::<Vec<_>>();
    let data = Matrix::new(8, board);
    let mut display = MatrixDisplay::new(format, data);
    display.print(&mut std::io::stdout(), &BoxStyle::Rounded);
}
