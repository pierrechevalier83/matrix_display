[![Build Status](https://travis-ci.org/pierrechevalier83/matrix_display.svg?branch=master)](https://travis-ci.org/pierrechevalier83/matrix_display)
[![Coverage Status](https://coveralls.io/repos/github/pierrechevalier83/matrix_display/badge.svg?branch=master)](https://coveralls.io/github/pierrechevalier83/matrix_display?branch=master)
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
- `cargo run --example 2048`

Example: visualising a chess board
----------------------------------

```
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
		    let mut ansi_bg = 0;
		    if i % 2 + (i / 8) % 2 == 1 {
			    ansi_bg = 7;
			}
			let ansi_fg = 28;
		    Cell::new(x.clone(), 28, ansi_bg)
			})
        .collect::<Vec<_>>();
    let data = Matrix::new(8, board);
    let mut display = MatrixDisplay::new(format, data);
    display.print(&mut std::io::stdout(), &BoxStyle::Rounded);
}
```

