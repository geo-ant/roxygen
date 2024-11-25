use roxygen::*;

#[roxygen]
/// here are some comments
#[roxygen]
/// and some more
#[parameters_section]
/// and some more
/// but this next argument section should not be here
#[parameters_section]
pub fn add(
    /// some comments
    first: i32,
    second: i32,
) -> i32 {
    first + second
}

pub fn main() {}
