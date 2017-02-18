pub use self::position::Position;

mod position {

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
    pub fn top(&self) -> bool {
        match *self {
            Position::TopLeft => true,
            Position::Top => true,
            Position::TopRight => true,
            _ => false,
        }
    }
    pub fn left(&self) -> bool {
        match *self {
            Position::TopLeft => true,
            Position::Left => true,
            Position::BottomLeft => true,
            _ => false,
        }
    }
    pub fn right(&self) -> bool {
        match *self {
            Position::TopRight => true,
            Position::Right => true,
            Position::BottomRight => true,
            _ => false,
        }
    }
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
