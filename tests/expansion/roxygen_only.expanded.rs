use roxygen::*;
/// this is documentation
/// and this is too
///
///**Arguments**:
///
/// * `bar`: this has one line of docs
/// * `baz`: this has
///    two lines of docs
fn foo(bar: u32, baz: String, _undocumented: i32) -> bool {
    baz.len() > bar as usize
}
