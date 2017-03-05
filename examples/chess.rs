extern crate matrix_display;
use matrix_display::*;

fn main() {
    let format = Format::new(7, 3);
    #[cfg_attr(rustfmt, rustfmt_skip)]
    let board = vec!['♜', '♞', '♝', '♛', '♚', '♝', '♞', '♜',
	                 '♟', '♟', '♟', '♟', '♟', '♟', '♟', '♟',
					 ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                     ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
					 ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
					 ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
					 '♖', '♘', '♗', '♕', '♔', '♗', '♘', '♖',
					 '♙', '♙', '♙', '♙', '♙', '♙', '♙', '♙']
        .iter()
        .enumerate()
        .map(|(i, x)| {
            let ansi_fg = 33;
            let mut ansi_bg = 0;
            if i % 2 + (i / 8) % 2 == 1 {
                ansi_bg = 7;
            }
            cell::Cell::new(x.clone(), ansi_fg, ansi_bg)
        })
        .collect::<Vec<_>>();
    let data = matrix::Matrix::new(8, board);
    let display = MatrixDisplay::new(format, data);
    display.print(&mut std::io::stdout(), &style::BordersStyle::None);
}
