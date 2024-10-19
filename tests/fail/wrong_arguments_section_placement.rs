use doxidize::*;

/// here are some comments
/// this arguments section should not be here
#[arguments_section]
/// and some more
#[doxidize]
pub fn add(
    /// some comments
    first: i32,
    second: i32,
) -> i32 {
    first + second
}

pub fn main() {}
