use roxygen::*;
/// this is documentation
/// and this is too
///
/// **Arguments**:
///
/// * `bar`: this has one line of docs
/// * `baz`: this has
///    two lines of docs
///
/// **Generics**:
///
/// * `T`: documentation for parameter T
///    spans multiple lines
///
/// this goes after the arguments section
fn foo<S, T>(bar: u32, baz: String, _undocumented: i32) -> bool {
    baz.len() > bar as usize
}
