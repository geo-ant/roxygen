use roxygen::*;

#[roxygen]
/// this is documentation
/// and this is too
#[parameters_section]
/// this goes after the arguments section
fn foo<
    /// a lifetime
    'a,
    S,
    /// documentation for parameter T
    /// spans multiple lines
    T,
    /// a const generic
    const N: usize,
>(
    /// this has one line of docs
    bar: u32,
    /// this has
    /// two lines of docs
    baz: String,
    _undocumented: i32,
) -> bool {
    baz.len() > bar as usize
}
