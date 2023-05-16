//! # chardet14_release_profiles
//!
//! 'chardet14_release_profiles' is a collection of utilities to make performing certain
//! calculations more convenient
//!

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
        PUrple,
    }
}

pub mod utils {
    use super::kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        SecondaryColor::Orange
    }
}
/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = chardet14_release_profiles::add_one(arg);
///
/// assert_eq!(6,answer);
/// ```

pub fn add_one(x: i32) -> i32 {
    x + 1
}
