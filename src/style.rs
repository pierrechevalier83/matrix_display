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
    Light,
    /// ╭─┬─╮
    /// │✓│✓│
    /// ├─┼─┤
    /// │✓│✓│
    /// ╰─┴─╯
    ArcLight,
    /// ┏━┳━┓
    /// ┃✓┃✓┃
    /// ┣━╋━┫
    /// ┃✓┃✓┃
    /// ┗━┻━┛
    Heavy,
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
            BordersStyle::Light => BoxDrawing::LightDownAndRight.into(),
            BordersStyle::ArcLight => BoxDrawing::LightArcDownAndRight.into(),
            BordersStyle::Heavy => BoxDrawing::HeavyDownAndRight.into(),
            BordersStyle::Double => BoxDrawing::DoubleDownAndRight.into(),
        }
    }
    pub fn top_right_corner(&self) -> char {
        match *self {
            BordersStyle::None => '\0',
            BordersStyle::Plain => ' ',
            BordersStyle::Retro => '+',
            BordersStyle::Light => BoxDrawing::LightDownAndLeft.into(),
            BordersStyle::ArcLight => BoxDrawing::LightArcDownAndLeft.into(),
            BordersStyle::Heavy => BoxDrawing::HeavyDownAndLeft.into(),
            BordersStyle::Double => BoxDrawing::DoubleDownAndLeft.into(),
        }
    }
    pub fn bottom_left_corner(&self) -> char {
        match *self {
            BordersStyle::None => '\0',
            BordersStyle::Plain => ' ',
            BordersStyle::Retro => '+',
            BordersStyle::Light => BoxDrawing::LightUpAndRight.into(),
            BordersStyle::ArcLight => BoxDrawing::LightArcUpAndRight.into(),
            BordersStyle::Heavy => BoxDrawing::HeavyUpAndRight.into(),
            BordersStyle::Double => BoxDrawing::DoubleUpAndRight.into(),
        }
    }
    pub fn bottom_right_corner(&self) -> char {
        match *self {
            BordersStyle::None => '\0',
            BordersStyle::Plain => ' ',
            BordersStyle::Retro => '+',
            BordersStyle::Light => BoxDrawing::LightUpAndLeft.into(),
            BordersStyle::ArcLight => BoxDrawing::LightArcUpAndLeft.into(),
            BordersStyle::Heavy => BoxDrawing::HeavyUpAndLeft.into(),
            BordersStyle::Double => BoxDrawing::DoubleUpAndLeft.into(),
        }
    }
    pub fn horizontal_border(&self) -> char {
        match *self {
            BordersStyle::None => '\0',
            BordersStyle::Plain => ' ',
            BordersStyle::Retro => '-',
            BordersStyle::Light | BordersStyle::ArcLight => BoxDrawing::LightHorizontal.into(),
            BordersStyle::Heavy => BoxDrawing::HeavyHorizontal.into(),
            BordersStyle::Double => BoxDrawing::DoubleHorizontal.into(),
        }
    }
    pub fn top_intersection(&self) -> char {
        match *self {
            BordersStyle::None => '\0',
            BordersStyle::Plain => ' ',
            BordersStyle::Retro => '+',
            BordersStyle::Light | BordersStyle::ArcLight => {
                BoxDrawing::LightDownAndHorizontal.into()
            }
            BordersStyle::Heavy => BoxDrawing::HeavyDownAndHorizontal.into(),
            BordersStyle::Double => BoxDrawing::DoubleDownAndHorizontal.into(),
        }
    }
    pub fn left_intersection(&self) -> char {
        match *self {
            BordersStyle::None => '\0',
            BordersStyle::Plain => ' ',
            BordersStyle::Retro => '+',
            BordersStyle::Light | BordersStyle::ArcLight => {
                BoxDrawing::LightVerticalAndRight.into()
            }
            BordersStyle::Heavy => BoxDrawing::HeavyVerticalAndRight.into(),
            BordersStyle::Double => BoxDrawing::DoubleVerticalAndRight.into(),
        }
    }
    pub fn right_intersection(&self) -> char {
        match *self {
            BordersStyle::None => '\0',
            BordersStyle::Plain => ' ',
            BordersStyle::Retro => '+',
            BordersStyle::Light | BordersStyle::ArcLight => BoxDrawing::LightVerticalAndLeft.into(),
            BordersStyle::Heavy => BoxDrawing::HeavyVerticalAndLeft.into(),
            BordersStyle::Double => BoxDrawing::DoubleVerticalAndLeft.into(),
        }
    }
    pub fn bottom_intersection(&self) -> char {
        match *self {
            BordersStyle::None => '\0',
            BordersStyle::Plain => ' ',
            BordersStyle::Retro => '+',
            BordersStyle::Light | BordersStyle::ArcLight => BoxDrawing::LightUpAndHorizontal.into(),
            BordersStyle::Heavy => BoxDrawing::HeavyUpAndHorizontal.into(),
            BordersStyle::Double => BoxDrawing::DoubleUpAndHorizontal.into(),
        }
    }
    pub fn intersection(&self) -> char {
        match *self {
            BordersStyle::None => '\0',
            BordersStyle::Plain => ' ',
            BordersStyle::Retro => '+',
            BordersStyle::Light | BordersStyle::ArcLight => {
                BoxDrawing::LightVerticalAndHorizontal.into()
            }
            BordersStyle::Heavy => BoxDrawing::HeavyVerticalAndHorizontal.into(),
            BordersStyle::Double => BoxDrawing::DoubleVerticalAndHorizontal.into(),
        }
    }
    pub fn vertical_border(&self) -> char {
        match *self {
            BordersStyle::None => '\0',
            BordersStyle::Plain => ' ',
            BordersStyle::Retro => '|',
            BordersStyle::Light | BordersStyle::ArcLight => BoxDrawing::LightVertical.into(),
            BordersStyle::Heavy => BoxDrawing::HeavyVertical.into(),
            BordersStyle::Double => BoxDrawing::DoubleVertical.into(),
        }
    }
}
