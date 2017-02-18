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


#[derive(Clone, Debug, PartialEq)]
pub struct Cell<T>
    where T: Clone,
          T: ToString
{
    pub value: T,
    pub color: AnsiColor,
}
impl<T> Cell<T>
    where T: Clone,
          T: ToString
{
    pub fn new(val: T, fg: u8, bg: u8) -> Cell<T> {
        Cell {
            value: val,
            color: AnsiColor { fg: fg, bg: bg },
        }
    }
}
}
