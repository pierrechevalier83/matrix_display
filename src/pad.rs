pub use self::pad::Pad;
pub use self::pad::horizontal_pad;

extern crate unicode_segmentation;



#[cfg(test)]
mod pad_tests {
    use super::Pad;
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
}

mod pad {

use pad::unicode_segmentation::UnicodeSegmentation;
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

pub fn horizontal_pad(width: usize, s: &str, c: char) -> String {
    let pad = Pad::new(width, s.graphemes(true).count());
    std::iter::repeat(c).take(pad.before).collect::<String>() + s +
    &std::iter::repeat(c).take(pad.after).collect::<String>()
}
}
