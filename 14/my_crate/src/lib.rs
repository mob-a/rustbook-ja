//! # My Crate
//!
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.
//!

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
///
/// Add one to the number given.
///
/// # Examples
///
/// ```
/// assert_eq!(my_crate::add_one(2), 3);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
