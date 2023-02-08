//! # cargo_crates_demo
//!
//! cargo_crates_demo是一系列工具的集合，
//! 这些工具被用来简化特定的计算操作

/// Adds one to the number given
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = cargo_crates_demo::add_one(arg);
///
/// assert_eq!(6, answer);
///
pub fn add_one(x: i32) -> i32 {
  x + 1
}


pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        SecondaryColor::Green
    }
}