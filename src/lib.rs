#![doc= include_str!("../Readme.md")]
//! ## Documenting Generics
//! Generic parameters can be documented with doc comments just as the arguments
//! can be:
//! ```rust
//! use roxygen::roxygen;
//!
//! #[roxygen]
//! fn frobnicate<
//! /// some comment goes here
//! S,
//! T> (
//! /// the value being frobnicated
//! frobnicator: T,
//! /// the frobnicant
//! frobnicant: S) -> T
//! {
//!    todo!()
//! }
//! ```
//! This generates an additional section for the generic parameters right
//! after the arguments section (if it exists).
//! All types of generic arguments, including lifetimes and const-generics
//! can be documented like this.
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Attribute, ItemFn};
use util::{
    extract_documented_generics, extract_documented_parameters, extract_fn_doc_attrs,
    make_doc_block,
};
mod util;

/// parameter section macro name
const PARAM_SECTION: &str = "parameters_section";
/// the name of this crate
const ROXYGEN_CRATE: &str = "roxygen";

// helper macro "try" on a syn::Error, so that we can return it as a token stream
macro_rules! try2 {
    ($ex:expr) => {
        match $ex {
            Ok(val) => val,
            Err(err) => return err.into_compile_error().into(),
        }
    };
}

#[proc_macro_attribute]
/// the principal attribute inside this crate that lets us document function arguments
pub fn roxygen(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut function: ItemFn = parse_macro_input!(item as ItemFn);

    try2!(function.attrs.iter_mut().try_for_each(|attr| {
        if is_roxygen_main(attr) {
            Err(syn::Error::new_spanned(
                attr,
                "Duplicate attribute. This attribute must only appear once.",
            ))
        } else {
            Ok(())
        }
    }));

    // extrac the doc attributes on the function itself
    let function_docs = try2!(extract_fn_doc_attrs(&mut function.attrs));

    let documented_params = try2!(extract_documented_parameters(
        function.sig.inputs.iter_mut()
    ));

    let documented_generics = try2!(extract_documented_generics(&mut function.sig.generics));

    let has_documented_params = !documented_params.is_empty();
    let has_documented_generics = !documented_generics.is_empty();

    if !has_documented_params && !has_documented_generics {
        return syn::Error::new_spanned(
            function.sig.ident,
            "Function has no documented parameters or generics.\nDocument at least one function parameter or generic.",
        )
        .into_compile_error()
        .into();
    }

    let parameter_doc_block = make_doc_block("Parameters", documented_params);
    let generics_doc_block = make_doc_block("Generics", documented_generics);

    let docs_before = function_docs.before_args_section;
    let docs_after = function_docs.after_args_section;
    let maybe_empty_doc_line = if !docs_after.is_empty() {
        Some(quote! {#[doc=""]})
    } else {
        None
    };

    quote! {
        #(#docs_before)*
        #parameter_doc_block
        #generics_doc_block
        #maybe_empty_doc_line
        #(#docs_after)*
        #function
    }
    .into()
}

// this is to expose the helper attribute #[arguments_section].
// The only logic about this attribute that this here function includes is
// to make sure that this attribute is not placed before the #[roxygen]
// attribute. All other logic is handled in the roxygen macro itself.
/// a helper attribute that dictates the placement of the section documenting
/// the function arguments
#[proc_macro_attribute]
pub fn parameters_section(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let function: ItemFn = parse_macro_input!(item as ItemFn);

    // enforce that this macro comes after roxygen, which means it
    // cannot see the roxygen attribute
    let maybe_roxygen = function.attrs.iter().find(|attr| is_roxygen_main(attr));
    if let Some(attr) = maybe_roxygen {
        syn::Error::new_spanned(attr,"The #[roxygen] attribute must come before the arguments section attribute.\nPlace it before any of the doc comments for the function.").into_compile_error().into()
    } else {
        function.to_token_stream().into()
    }
}

/// check whether an attribute is the arguments section attribute.
/// Stick this into it's own function so I can change the logic
//@note(geo) this logic won't work if the crate is renamed
#[inline(always)]
fn is_parameters_section(attr: &Attribute) -> bool {
    let path = attr.path();

    if path.is_ident(PARAM_SECTION) {
        true
    } else {
        // checks for (::)roxygen::param_section
        path.segments.len() == 2
            && path.segments[0].ident == ROXYGEN_CRATE
            && path.segments[1].ident == PARAM_SECTION
    }
}

/// check whether an attribute is the raw #[roxygen] main attribute.
/// Stuck into this function, so I can refactor this logic
#[inline(always)]
fn is_roxygen_main(attr: &Attribute) -> bool {
    attr.path().is_ident("roxygen")
}
