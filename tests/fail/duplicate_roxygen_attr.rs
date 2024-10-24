use roxygen::*;

#[roxygen]
/// here are some comments
#[roxygen]
/// and some more
#[arguments_section]
/// and some more
/// but this next argument section should not be here
#[arguments_section]
pub fn add(
    /// some comments
    first: i32,
    second: i32,
) -> i32 {
    first + second
}

pub fn main() {}
