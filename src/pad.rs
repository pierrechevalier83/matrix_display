pub use self::pad::horizontal_pad;
pub use self::pad::Pad;

extern crate unicode_width;

#[cfg(test)]
mod pad_tests {
    use super::{horizontal_pad, Pad};
    #[test]
    pub fn adds_up_to_correct_size() {
        let pad = Pad::new(12, 3);
        assert_eq!(12, pad.before + 3 + pad.after);
    }
    #[test]
    pub fn centered() {
        let pad = Pad::new(10, 2);
        assert_eq!(4, pad.before);
        assert_eq!(4, pad.after);
    }
    #[test]
    pub fn odd_total() {
        let pad = Pad::new(11, 3);
        assert_eq!(4, pad.before);
        assert_eq!(4, pad.after);
    }
    #[test]
    pub fn no_pad_even() {
        let pad = Pad::new(4, 4);
        assert_eq!(0, pad.before);
        assert_eq!(0, pad.after);
    }
    #[test]
    pub fn no_pad_odd() {
        let pad = Pad::new(1, 1);
        assert_eq!(0, pad.before);
        assert_eq!(0, pad.after);
    }
    #[test]
    pub fn pad_ascii() {
        let output = horizontal_pad(8, "ascii", ' ');
        assert_eq!(output, "  ascii ");
    }
    #[test]
    pub fn pad_cjk() {
        let output = horizontal_pad(8, "中文", ' ');
        assert_eq!(output, "  中文  ");
    }
}

mod pad {

    use pad::unicode_width::{UnicodeWidthChar, UnicodeWidthStr};
    use std;

    pub struct Pad {
        pub before: usize,
        pub after: usize,
    }
    impl Pad {
        pub fn new(total: usize, content: usize) -> Pad {
            Pad {
                before: (total - content) / 2 + (total - content) % 2,
                after: (total - content) / 2,
            }
        }
    }

    /// ## Panic When:
    /// - pad char width > 1.
    pub fn horizontal_pad(width: usize, s: &str, c: char) -> String {
        assert!(c.width().unwrap_or(0) <= 1, "{:?} width > 1", c);

        let pad = Pad::new(width, s.width());
        std::iter::repeat(c).take(pad.before).collect::<String>()
            + s
            + &std::iter::repeat(c).take(pad.after).collect::<String>()
    }
}
