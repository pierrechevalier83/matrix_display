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
            let ansi_fg = 28;
			let mut ansi_bg = 0;
		    if i % 2 + (i / 8) % 2 == 1 {
			    ansi_bg = 7;
			}
		    cell::Cell::new(x.clone(), ansi_fg, ansi_bg)
			})
        .collect::<Vec<_>>();
    let data = matrix::Matrix::new(8, board);
    let display = MatrixDisplay::new(format, data);
    display.print(&mut std::io::stdout(), &style::BordersStyle::Rounded);
}
```
![alt tag](https://github.com/pierrechevalier83/matrix_display/blob/master/screenshots/chess.png)

Example: visualising a 2048 game
--------------------------------

```
extern crate matrix_display;
use matrix_display::*;

fn main() {
    let format = Format::new(7, 3);
    let colour_theme = vec![247, 78, 222, 220, 214, 208, 202, 196, 162, 160, 126, 90, 88, 54, 53, 52];
    let board = (0..16)
        .map(|x| {
            cell::Cell::new(2_f64.powi(x + 1),
                      7,
                      *colour_theme.get(x as usize).unwrap() as u8)
        })
        .collect::<Vec<_>>();
    let data = matrix::Matrix::new(4, board);
    let display = MatrixDisplay::new(format, data);
    display.print(&mut std::io::stdout(), &style::BordersStyle::Thick);
}
```

![alt tag](https://github.com/pierrechevalier83/matrix_display/blob/master/screenshots/2048.png)

Example: visualising a 256 colors palette
-----------------------------------------

```
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
```

![alt tag](https://github.com/pierrechevalier83/matrix_display/blob/master/screenshots/palette.png)
