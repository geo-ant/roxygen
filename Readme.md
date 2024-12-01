# roxygen - documenting function parameters

![build](https://github.com/geo-ant/roxygen/actions/workflows/build.yml/badge.svg?branch=main)
![tests](https://github.com/geo-ant/roxygen/actions/workflows/tests.yml/badge.svg?branch=main)
![lints](https://github.com/geo-ant/roxygen/actions/workflows/lints.yml/badge.svg?branch=main)
[![crates](https://img.shields.io/crates/v/roxygen)](https://crates.io/crates/roxygen)
![maintenance-status](https://img.shields.io/badge/maintenance-passively--maintained-yellowgreen.svg)
[![crates](https://raw.githubusercontent.com/geo-ant/user-content/refs/heads/main/ko-fi-support.svg)](https://ko-fi.com/geoant)

The `#[roxygen]` attribute allows you to add doc-comments to function
parameters, which is a _compile error_ in current Rust. Generic lifetimes,
types, and constants of the function [can also be documented](https://docs.rs/roxygen/latest/roxygen/). 
You can now write

```rust
use roxygen::*;

#[roxygen]
/// sum the rows of an image
fn sum_image_rows(
  /// the image data in row-major format
  image_data: &[f32],
  /// the number of rows in the image
  nrows: u32,
  /// the number of columns in the image
  ncols: u32,
  /// an out buffer into which the resulting
  /// sums are placed. Must have space 
  /// for exactly `nrows` elements
  sums: &mut [f32]) -> Result<(),String> {
    todo!()
} 
```

You have to document at least one parameter (or generic), but you don't have
to document all of them. The example above will produce documentation as 
if you had written a doc comment for the function like so:

```rust
/// sum the rows of an image
///
/// **Parameters**: 
///
/// * `image_data`: the image data in row-major format
/// * `nrows`: the number of rows in the image
/// * `ncols`: the number of columns in the image
/// * `sums`: an out buffer into which the resulting
///    sums are placed. Must have space 
///    for exactly `nrows` elements
fn sum_image_rows(
  image_data: &[f32],
  nrows: u32,
  ncols: u32,
  sums: &mut [f32]) -> Result<(),String> {
    todo!()
}
```

⚠️  **Renaming** the macros exported from this crate (`use ... as ...`) or renaming the
crate itself (in your `Cargo.toml`) will make all of this stop working properly.

## Placing the Parameters-Section

By default, the section documenting the parameters will go at the end
of the top-level function documentation. However, this crate allows to explicitly
place the section by using a custom attribute like so:

```rust
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
  second: f32
  )
{}
```

## Considerations

It's a [long standing issue](https://github.com/rust-lang/rust/issues/57525)
whether and how to add this capability to `rustdoc`. Firstly, there's no
general consensus on how exactly to document function parameters. However, 
I've seen the presented style used a lot, with minor variations.
Secondly, the standard library [doesn't need this](https://github.com/rust-lang/rust/issues/57525#issuecomment-453633783)
style of documentation at all. So before you stick this macro on every function,
do consider

* taking inspiration from how the standard library deals with function parameters,
* using fewer function parameters,
* using more descriptive parameters names,
* using _types_ to communicate intent,
* sticking function parameters in a `struct`.

Here is [an elegant way](https://www.reddit.com/r/rust/comments/1gb782e/comment/ltpk16x/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button),
how the example above can be reworked without using per parameter documentation:

```rust
/// Sums the rows of an image.
///
/// The rows of `image_data`, an `nrows` by `ncols`
/// matrix in row-major ordering, are summed into `sums`
/// which must have exactly `nrows` elements.
fn sum_image_rows(
  image_data: &[f32],
  nrows: u32,
  ncols: u32,
  sums: &mut [f32]) -> Result<(),String> {
    todo!()
}
```

All that being said, I've realized that sometimes I still want to document
function parameters.

### Compile Times

Macros will always increase your compile time to some degree, but I don't think
this is a giant issue (after the roxygen dependency itself was compiled, that is):
firstly, this macro is to be used _sparingly_. Secondly, this macro just does 
some light parsing and shuffling around of the documentation tokens. It 
introduces no additional code. Thus, it doesn't
make your actual code more or less complex and should not affect compile
times much (after this crate was compiled once), but I haven't
measured it... so take it with a grain of sodium-chloride.
