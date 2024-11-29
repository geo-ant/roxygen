use roxygen::roxygen;

#[roxygen]
/// long documention
/// ...
#[roxygen::parameters_section]
/// # Examples
/// ...
fn foo(
    /// some docs
    first: i32,
    second: f32,
) {
}
