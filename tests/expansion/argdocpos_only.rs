use roxygen::*;

#[argdocpos]
fn foo(
    /// this is documentation
    /// and this is too
    // but this is not
    bar: u32, /// this has one line of docs
    baz: String, /// this has
    /// two lines of docs
    _undocumented: i32,
) -> bool {
    baz.len() > bar as usize
}
