Matrix Display
--------------
A simple rust library to visualize 2D matrixes in rust.

- Supports 256 terminal colours using AnsiTerm
- Multiple unicode box character sets supported (plain, retro, thin, rounded, thick, double)

Build
-----
- `git clone git@github.com:pierrechevalier83/matrix_display.git`
- `cargo test`

Run
---
- `cargo run --example chess`

Example: visualising a chess board
----------------------------------

```
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
    display.print(&mut std::io::stdout(), &BoxStyle::Rounded);
    println!();
}
```
