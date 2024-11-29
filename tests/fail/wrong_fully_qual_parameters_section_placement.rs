/// here are some comments
/// this arguments section should not be here
#[roxygen::parameters_section]
/// and some more
#[roxygen::roxygen]
pub fn add(
    /// some comments
    first: i32,
    second: i32,
) -> i32 {
    first + second
}

pub fn main() {}
