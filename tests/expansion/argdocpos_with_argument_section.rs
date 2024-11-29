use roxygen::*;

#[roxygen]
/// this is documentation
/// and this is too
#[parameters_section]
fn foo( /// this goes after the arguments section
    bar: u32, /// this has one line of docs
    baz: String, /// this has
    /// two lines of docs
    _undocumented: i32,
) -> bool {
    baz.len() > bar as usize
}
