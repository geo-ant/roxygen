use roxygen::*;

trait Foo {
    #[roxygen]
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

    /// I almost forgot the whole point was to test
    /// whether it works for functions without body...
    /// it should...
    ///
    /// **Parameters**:
    ///
    /// * `first`: the first parameter
    /// * `second`: the second, more interesting parameter
    ///    with more interesting
    ///
    ///    ```rust
    ///      docs = very_interesting!();
    ///    ```
    ///
    ///    Sorry it's kinda late and I'm tired... but my professional
    ///    honor is not letting me skip this test.
    fn bar(first: i32, second: f32);
}
