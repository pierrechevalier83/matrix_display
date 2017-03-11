//! A simple human readable way of defining a position within a box
pub use self::position::Position;

mod position {
    /// Enum that describes a position within a rectangle
	///
	/// The position can be accessed at two levels of granularity:
	/// The values of the enum denote the exact position (i.e: `BottomLeft`)
	/// Helper functions help determine wether that position is part of a given border
	/// (i.e: `BottomLeft` is both part of `bottom()` and `left()`)
    #[derive(Debug, PartialEq)]
    pub enum Position {
        Top,
        Left,
        Right,
        Bottom,
        TopLeft,
        TopRight,
        BottomLeft,
        BottomRight,
        Middle,
    }

    impl Position {
	    /// Is it part of the top row?
        pub fn top(&self) -> bool {
            match *self {
                Position::TopLeft => true,
                Position::Top => true,
                Position::TopRight => true,
                _ => false,
            }
        }
	    /// Is it part of the leftmost column?
        pub fn left(&self) -> bool {
            match *self {
                Position::TopLeft => true,
                Position::Left => true,
                Position::BottomLeft => true,
                _ => false,
            }
        }
	    /// Is it part of the rightmost column?
        pub fn right(&self) -> bool {
            match *self {
                Position::TopRight => true,
                Position::Right => true,
                Position::BottomRight => true,
                _ => false,
            }
        }
	    /// Is it part of the bottom row?
        pub fn bottom(&self) -> bool {
            match *self {
                Position::BottomLeft => true,
                Position::Bottom => true,
                Position::BottomRight => true,
                _ => false,
            }
        }
    }
}
