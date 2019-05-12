//! Personalise the border style of your matric display
//!
//! We offer a convenient enum to describe different unicode
//! borders so you don't have to hunt through unicode tables
//! and copy paste unicode symbols like an animal!

use unicode_types::box_drawing::BoxDrawing;

/// An enum that represents different unicode borders styles
///
/// It provides accessors for specific locations such as top_left_corner to be used by the printing code
#[derive(PartialEq)]
pub enum BordersStyle {
    /// ✓✓
    /// ✓✓
    None,
    ///
    ///  ✓ ✓
    ///
    ///  ✓ ✓
    ///
    Plain,
    /// +-+-+
    /// |✓|✓|
    /// +-+-+
    /// |✓|✓|
    /// +-+-+
    Retro,
    /// ┌─┬─┐
    /// │✓│✓│
    /// ├─┼─┤
    /// │✓│✓│
    /// └─┴─┘
    Thin,
    /// ╭─┬─╮
    /// │✓│✓│
    /// ├─┼─┤
    /// │✓│✓│
    /// ╰─┴─╯
    Rounded,
    /// ┏━┳━┓
    /// ┃✓┃✓┃
    /// ┣━╋━┫
    /// ┃✓┃✓┃
    /// ┗━┻━┛
    Thick,
    /// ╔═╦═╗
    /// ║✓║✓║
    /// ╠═╬═╣
    /// ║✓║✓║
    /// ╚═╩═╝
    Double,
}

impl BordersStyle {
    pub fn top_left_corner(&self) -> char {
        match *self {
            BordersStyle::None => '\0',
            BordersStyle::Plain => ' ',
            BordersStyle::Retro => '+',
            BordersStyle::Thin => BoxDrawing::LightDownAndRight.into(),
            BordersStyle::Rounded => BoxDrawing::LightArcDownAndRight.into(),
            BordersStyle::Thick => BoxDrawing::HeavyDownAndRight.into(),
            BordersStyle::Double => BoxDrawing::DoubleDownAndRight.into(),
        }
    }
    pub fn top_right_corner(&self) -> char {
        match *self {
            BordersStyle::None => '\0',
            BordersStyle::Plain => ' ',
            BordersStyle::Retro => '+',
            BordersStyle::Thin => BoxDrawing::LightDownAndLeft.into(),
            BordersStyle::Rounded => BoxDrawing::LightArcDownAndLeft.into(),
            BordersStyle::Thick => BoxDrawing::HeavyDownAndLeft.into(),
            BordersStyle::Double => BoxDrawing::DoubleDownAndLeft.into(),
        }
    }
    pub fn bottom_left_corner(&self) -> char {
        match *self {
            BordersStyle::None => '\0',
            BordersStyle::Plain => ' ',
            BordersStyle::Retro => '+',
            BordersStyle::Thin => BoxDrawing::LightUpAndRight.into(),
            BordersStyle::Rounded => BoxDrawing::LightArcUpAndRight.into(),
            BordersStyle::Thick => BoxDrawing::HeavyUpAndRight.into(),
            BordersStyle::Double => BoxDrawing::DoubleUpAndRight.into(),
        }
    }
    pub fn bottom_right_corner(&self) -> char {
        match *self {
            BordersStyle::None => '\0',
            BordersStyle::Plain => ' ',
            BordersStyle::Retro => '+',
            BordersStyle::Thin => BoxDrawing::LightUpAndLeft.into(),
            BordersStyle::Rounded => BoxDrawing::LightArcUpAndLeft.into(),
            BordersStyle::Thick => BoxDrawing::HeavyUpAndLeft.into(),
            BordersStyle::Double => BoxDrawing::DoubleUpAndLeft.into(),
        }
    }
    pub fn horizontal_border(&self) -> char {
        match *self {
            BordersStyle::None => '\0',
            BordersStyle::Plain => ' ',
            BordersStyle::Retro => '-',
            BordersStyle::Thin => BoxDrawing::LightHorizontal.into(),
            BordersStyle::Rounded => BoxDrawing::LightHorizontal.into(),
            BordersStyle::Thick => BoxDrawing::HeavyHorizontal.into(),
            BordersStyle::Double => BoxDrawing::DoubleHorizontal.into(),
        }
    }
    pub fn top_intersection(&self) -> char {
        match *self {
            BordersStyle::None => '\0',
            BordersStyle::Plain => ' ',
            BordersStyle::Retro => '+',
            BordersStyle::Thin => BoxDrawing::LightDownAndHorizontal.into(),
            BordersStyle::Rounded => BoxDrawing::LightDownAndHorizontal.into(),
            BordersStyle::Thick => BoxDrawing::HeavyDownAndHorizontal.into(),
            BordersStyle::Double => BoxDrawing::DoubleDownAndHorizontal.into(),
        }
    }
    pub fn left_intersection(&self) -> char {
        match *self {
            BordersStyle::None => '\0',
            BordersStyle::Plain => ' ',
            BordersStyle::Retro => '+',
            BordersStyle::Thin => BoxDrawing::LightVerticalAndRight.into(),
            BordersStyle::Rounded => BoxDrawing::LightVerticalAndRight.into(),
            BordersStyle::Thick => BoxDrawing::HeavyVerticalAndRight.into(),
            BordersStyle::Double => BoxDrawing::DoubleVerticalAndRight.into(),
        }
    }
    pub fn right_intersection(&self) -> char {
        match *self {
            BordersStyle::None => '\0',
            BordersStyle::Plain => ' ',
            BordersStyle::Retro => '+',
            BordersStyle::Thin => BoxDrawing::LightVerticalAndLeft.into(),
            BordersStyle::Rounded => BoxDrawing::LightVerticalAndLeft.into(),
            BordersStyle::Thick => BoxDrawing::HeavyVerticalAndLeft.into(),
            BordersStyle::Double => BoxDrawing::DoubleVerticalAndLeft.into(),
        }
    }
    pub fn bottom_intersection(&self) -> char {
        match *self {
            BordersStyle::None => '\0',
            BordersStyle::Plain => ' ',
            BordersStyle::Retro => '+',
            BordersStyle::Thin => BoxDrawing::LightUpAndHorizontal.into(),
            BordersStyle::Rounded => BoxDrawing::LightUpAndHorizontal.into(),
            BordersStyle::Thick => BoxDrawing::HeavyUpAndHorizontal.into(),
            BordersStyle::Double => BoxDrawing::DoubleUpAndHorizontal.into(),
        }
    }
    pub fn intersection(&self) -> char {
        match *self {
            BordersStyle::None => '\0',
            BordersStyle::Plain => ' ',
            BordersStyle::Retro => '+',
            BordersStyle::Thin => BoxDrawing::LightVerticalAndHorizontal.into(),
            BordersStyle::Rounded => BoxDrawing::LightVerticalAndHorizontal.into(),
            BordersStyle::Thick => BoxDrawing::HeavyVerticalAndHorizontal.into(),
            BordersStyle::Double => BoxDrawing::DoubleVerticalAndHorizontal.into(),
        }
    }
    pub fn vertical_border(&self) -> char {
        match *self {
            BordersStyle::None => '\0',
            BordersStyle::Plain => ' ',
            BordersStyle::Retro => '|',
            BordersStyle::Thin => BoxDrawing::LightVertical.into(),
            BordersStyle::Rounded => BoxDrawing::LightVertical.into(),
            BordersStyle::Thick => BoxDrawing::HeavyVertical.into(),
            BordersStyle::Double => BoxDrawing::DoubleVertical.into(),
        }
    }
}
