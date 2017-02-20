pub use self::style::BordersStyle;

mod style {

pub enum BordersStyle {
    Plain, /*
            * . .
            *
            * . .
            *
            * */
    Retro, /* +-+-+
            * |.|.|
            * +-+-+
            * |.|.|
            * +-+-+
            * */
    Thin, /* ┌─┬─┐
           * │.│.│
           * ├─┼─┤
           * │.│.│
           * └─┴─┘
           * */
    Rounded, /* ╭─┬─╮
              * │.│.│
              * ├─┼─┤
              * │.│.│
              * ╰─┴─╯
              * */
    Thick, /* ┏━┳━┓
            * ┃.┃.┃
            * ┣━╋━┫
            * ┃.┃.┃
            * ┗━┻━┛
            * */
    Double, /* ╔═╦═╗
             * ║.║.║
             * ╠═╬═╣
             * ║.║.║
             * ╚═╩═╝
             * */
}

impl BordersStyle {
    pub fn top_left_corner(&self) -> char {
        match *self {
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

