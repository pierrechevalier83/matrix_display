extern crate matrix_display;
use matrix_display::*;

fn main() {
    let format = Format::new(3, 1); //default();
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
		    let mut ansi_bg = 0;
		    if i % 2 + (i / 8) % 2 == 1 {
			    ansi_bg = 7;
			}
		    Cell::new(x.clone(), 0, ansi_bg)
			})
        .collect::<Vec<_>>();
    let data = Matrix::new(8, board);
    let mut display = MatrixDisplay::new(format, data);
    display.print(&mut std::io::stdout(), &BoxStyle::Plain);
    println!();
	display.print(&mut std::io::stdout(), &BoxStyle::Retro);
    println!();
    display.print(&mut std::io::stdout(), &BoxStyle::Thin);
    println!();
    display.print(&mut std::io::stdout(), &BoxStyle::Rounded);
    println!();
    display.print(&mut std::io::stdout(), &BoxStyle::Thick);
    println!();
    display.print(&mut std::io::stdout(), &BoxStyle::Double);
    println!();
}
