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
    let mut data = matrix::Matrix::new(8, board);
    let mut display = MatrixDisplay::new(&format, &mut data);
    display.cell_at_cursor_position((13, 6)).color.bg = 10;
    display.print(&mut std::io::stdout(), &style::BordersStyle::None);
}
