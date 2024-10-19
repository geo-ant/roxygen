use doxidize::*;

#[doxidize]
/// this is documentation
/// and this is too
// but this is not
fn foo(
    /// this has one line of docs
    bar: u32,
    /// this has
    /// two lines of docs
    baz: String,
    _undocumented: i32,
) -> bool {
    baz.len() > bar as usize
}
