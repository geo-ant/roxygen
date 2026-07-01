use roxygen::*;
struct Foo {
    something: i32,
}
impl Foo {
    /// this is documentation
    /// and this is too
    ///
    /// **Parameters**:
    ///
    /// * `bar`: this has one line of docs
    /// * `baz`: this has
    ///    two lines of docs
    fn foo(bar: u32, baz: String, _undocumented: i32) -> bool {
        baz.len() > bar as usize
    }
}
