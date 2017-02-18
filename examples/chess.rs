extern crate matrix_display;
use matrix_display::*;

fn main() {
    let format = Format::new(7, 3);
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
            let ansi_fg = 28;
			let mut ansi_bg = 0;
		    if i % 2 + (i / 8) % 2 == 1 {
			    ansi_bg = 7;
			}
		    Cell::new(x.clone(), ansi_fg, ansi_bg)
			})
        .collect::<Vec<_>>();
    let data = Matrix::new(8, board);
    let mut display = MatrixDisplay::new(format, data);
    display.print(&mut std::io::stdout(), &BoxStyle::Rounded);
    println!();
}
