use doxidize::doxidize;

// just a smoke test that the proc macro can indeed be used like this.
// the real tests are in the macro expansion tests.

#[doxidize]
/// hello
///      this
///          is doc
fn foo(
    /// some comments
    /// more comments
    first: i32,
    second: f32,
) -> f32 {
    first as f32 - second
}

#[test]
fn test_foo() {
    assert_eq!(foo(1, 3.), -2.);
}
