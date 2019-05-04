//! Stores data to be printed by MatrixDisplay plus colour metada
//!
//! - The data type itself is generic. Anything that implements the ToString and the Clone traits can do
//! - The foreground and background color are individually configurable for each cell

pub use self::cell::AnsiColor;
pub use self::cell::Cell;

#[cfg(test)]
mod cell_tests {
    use super::Cell;
    #[test]
    fn constructor() {
        let c = Cell::new('F', 42, 24);
        assert_eq!(c.value, 'F');
        assert_eq!(c.color.fg, 42);
        assert_eq!(c.color.bg, 24);
    }
    #[test]
    fn clone_and_partial_eq() {
        let c = Cell::new('F', 42, 12);
        let d = c.clone();
        assert_eq!(c, d);
    }
}

mod cell {
    /// A foreground and background color's ansi code
    ///
    /// # Example:
    /// `matrix_display::cell::AnsiColor{ fg: 7, bg: 0 }`
    /// represents a white on dark color
    #[derive(Clone, Debug, PartialEq)]
    pub struct AnsiColor {
        pub fg: u8,
        pub bg: u8,
    }
    impl Default for AnsiColor {
        fn default() -> AnsiColor {
            AnsiColor { fg: 7, bg: 0 }
        }
    }

    /// A Matrix Cell that owns some data, a background color and a foreground color
    ///
    /// The colors are stored by their ansi codes in an AnsiColor struct
    /// The data can be any type that is clonable and converts to a string
    ///
    /// # Example (a chess game's representation):
    /// ```
    /// let board = vec!['♜', '♞', '♝', '♛', '♚', '♝', '♞', '♜',
    ///                  '♟', '♟', '♟', '♟', '♟', '♟', '♟', '♟',
    ///                  ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
    ///                  ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
    ///                  ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
    ///                  ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
    ///                  '♖', '♘', '♗', '♕', '♔', '♗', '♘', '♖',
    ///                  '♙', '♙', '♙', '♙', '♙', '♙', '♙', '♙']
    ///     .iter()
    ///     .enumerate()
    ///     .map(|(i, x)| {
    ///        let ansi_fg = 33; // pawns in blue
    ///         let mut ansi_bg = 0;
    ///         if i % 2 + (i / 8) % 2 == 1 {
    ///             ansi_bg = 7;
    ///         }
    ///         matrix_display::cell::Cell::new(x.clone(), ansi_fg, ansi_bg)
    ///         })
    ///     .collect::<Vec<_>>();
    /// ```
    #[derive(Clone, Debug, PartialEq)]
    pub struct Cell<T>
    where
        T: Clone,
        T: ToString,
    {
        pub value: T,
        pub color: AnsiColor,
    }
    impl<T> Cell<T>
    where
        T: Clone,
        T: ToString,
    {
        /// Construct a cell with any data, and two ansi codes for the foreground and the background colors
        pub fn new(val: T, fg: u8, bg: u8) -> Cell<T> {
            Cell {
                value: val,
                color: AnsiColor { fg: fg, bg: bg },
            }
        }
    }
}
