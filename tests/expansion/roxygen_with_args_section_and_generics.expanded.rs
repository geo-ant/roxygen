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
/// * `a`: a lifetime
/// * `T`: documentation for parameter T
///    spans multiple lines
/// * `N`: a const generic
///
/// this goes after the arguments section
fn foo<'a, S, T, const N: usize>(bar: u32, baz: String, _undocumented: i32) -> bool {
    baz.len() > bar as usize
}
