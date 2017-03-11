//! Personalise the border style of your matric display
//!
//! We offer a convenient enum to describe different unicode
//! borders so you don't have to hunt through unicode tables
//! and copy paste unicode symbols like an animal!

pub use self::style::BordersStyle;

mod style {
    /// An enum that represents different unicode borders styles
	///
	/// It provides accessors for specific locations such as top_left_corner to be used by the printing code
    #[derive(PartialEq)]
    pub enum BordersStyle {
		///✓✓
		///
		///✓✓
        None,
	    ///
		///✓✓
		///
		///✓✓
		///
        Plain,
	    /// +-+-+
		///
        /// |✓|✓|
		///
        /// +-+-+
		///
        /// |✓|✓|
		///
        /// +-+-+
        Retro,
        /// ┌─┬─┐
		///
        /// │✓│✓│
		///
        /// ├─┼─┤
		///
        /// │✓│✓│
		///
        /// └─┴─┘
        Thin,
		/// ╭─┬─╮
		///
        /// │✓│✓│
		///
        /// ├─┼─┤
		///
        /// │✓│✓│
		///
        /// ╰─┴─╯
        Rounded,
		/// ┏━┳━┓
		///
        /// ┃✓┃✓┃
		///
        /// ┣━╋━┫
		///
        /// ┃✓┃✓┃
		///
        /// ┗━┻━┛
        Thick,
		/// ╔═╦═╗
		///
        /// ║✓║✓║
		///
        /// ╠═╬═╣
		///
        /// ║✓║✓║
		///
        /// ╚═╩═╝
		Double,
    }

    impl BordersStyle {
        pub fn top_left_corner(&self) -> char {
            match *self {
                BordersStyle::None => '\0',
                BordersStyle::Plain => ' ',
                BordersStyle::Retro => '+',
                BordersStyle::Thin => '┌',
                BordersStyle::Rounded => '╭',
                BordersStyle::Thick => '┏',
                BordersStyle::Double => '╔',
            }
        }
        pub fn top_right_corner(&self) -> char {
            match *self {
                BordersStyle::None => '\0',
                BordersStyle::Plain => ' ',
                BordersStyle::Retro => '+',
                BordersStyle::Thin => '┐',
                BordersStyle::Rounded => '╮',
                BordersStyle::Thick => '┓',
                BordersStyle::Double => '╗',
            }
        }
        pub fn bottom_left_corner(&self) -> char {
            match *self {
                BordersStyle::None => '\0',
                BordersStyle::Plain => ' ',
                BordersStyle::Retro => '+',
                BordersStyle::Thin => '└',
                BordersStyle::Rounded => '╰',
                BordersStyle::Thick => '┗',
                BordersStyle::Double => '╚',
            }
        }
        pub fn bottom_right_corner(&self) -> char {
            match *self {
                BordersStyle::None => '\0',
                BordersStyle::Plain => ' ',
                BordersStyle::Retro => '+',
                BordersStyle::Thin => '┘',
                BordersStyle::Rounded => '╯',
                BordersStyle::Thick => '┛',
                BordersStyle::Double => '╝',
            }
        }
        pub fn horizontal_border(&self) -> char {
            match *self {
                BordersStyle::None => '\0',
                BordersStyle::Plain => ' ',
                BordersStyle::Retro => '-',
                BordersStyle::Thin => '─',
                BordersStyle::Rounded => '─',
                BordersStyle::Thick => '━',
                BordersStyle::Double => '═',
            }
        }
        pub fn top_intersection(&self) -> char {
            match *self {
                BordersStyle::None => '\0',
                BordersStyle::Plain => ' ',
                BordersStyle::Retro => '+',
                BordersStyle::Thin => '┬',
                BordersStyle::Rounded => '┬',
                BordersStyle::Thick => '┳',
                BordersStyle::Double => '╦',
            }
        }
        pub fn left_intersection(&self) -> char {
            match *self {
                BordersStyle::None => '\0',
                BordersStyle::Plain => ' ',
                BordersStyle::Retro => '+',
                BordersStyle::Thin => '├',
                BordersStyle::Rounded => '├',
                BordersStyle::Thick => '┣',
                BordersStyle::Double => '╠',
            }
        }
        pub fn right_intersection(&self) -> char {
            match *self {
                BordersStyle::None => '\0',
                BordersStyle::Plain => ' ',
                BordersStyle::Retro => '+',
                BordersStyle::Thin => '┤',
                BordersStyle::Rounded => '┤',
                BordersStyle::Thick => '┫',
                BordersStyle::Double => '╣',
            }
        }
        pub fn bottom_intersection(&self) -> char {
            match *self {
                BordersStyle::None => '\0',
                BordersStyle::Plain => ' ',
                BordersStyle::Retro => '+',
                BordersStyle::Thin => '┴',
                BordersStyle::Rounded => '┴',
                BordersStyle::Thick => '┻',
                BordersStyle::Double => '╩',
            }
        }
        pub fn intersection(&self) -> char {
            match *self {
                BordersStyle::None => '\0',
                BordersStyle::Plain => ' ',
                BordersStyle::Retro => '+',
                BordersStyle::Thin => '┼',
                BordersStyle::Rounded => '┼',
                BordersStyle::Thick => '╋',
                BordersStyle::Double => '╬',
            }
        }
        pub fn vertical_border(&self) -> char {
            match *self {
                BordersStyle::None => '\0',
                BordersStyle::Plain => ' ',
                BordersStyle::Retro => '|',
                BordersStyle::Thin => '│',
                BordersStyle::Rounded => '│',
                BordersStyle::Thick => '┃',
                BordersStyle::Double => '║',
            }
        }
    }
}
