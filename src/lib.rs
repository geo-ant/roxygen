#![doc= include_str!("../Readme.md")]
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Attribute, ItemFn};
use util::{extract_documented_parameters, extract_fn_doc_attrs, prepend_to_doc_attribute};
mod util;

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

    // will contain the docs comments for each documented function parameter
    // together with the identifier of the function parameter.
    let documented_params = try2!(extract_documented_parameters(
        function.sig.inputs.iter_mut()
    ));

    if documented_params.is_empty() {
        return syn::Error::new_spanned(
            function.sig.ident,
            "Function has no documented arguments.\nDocument at least one function argument.",
        )
        .into_compile_error()
        .into();
    }

    let parameter_doc_blocks = documented_params.into_iter().map(|param| {
        let mut docs_iter = param.docs.iter();
        // we always have at least one doc attribute because otherwise we
        // would not have inserted this pair into the parameter docs in the
        // first place
        let first = docs_iter
            .next()
            .expect("unexpectedly encountered empty doc list");

        let first_line = prepend_to_doc_attribute(&format!(" * `{}`:", param.ident), first);

        // we just need to indent the other lines, if they exist
        let next_lines = docs_iter.map(|attr| prepend_to_doc_attribute("   ", attr));
        quote! {
            #first_line
            #(#next_lines)*
        }
    });

    let docs_before = function_docs.before_args_section;
    let docs_after = function_docs.after_args_section;
    let maybe_empty_doc_line = if !docs_after.is_empty() {
        Some(quote! {#[doc=""]})
    } else {
        None
    };

    quote! {
        #(#docs_before)*
        #[doc=""]
        #[doc=" **Arguments**: "]
        #[doc=""]
        #(#parameter_doc_blocks)*
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
pub fn arguments_section(
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
#[inline(always)]
fn is_arguments_section(attr: &Attribute) -> bool {
    attr.path().is_ident("arguments_section")
}

/// check whether an attribute is the raw #[roxygen] main attribute.
/// Stuck into this function, so I can refactor this logic
#[inline(always)]
fn is_roxygen_main(attr: &Attribute) -> bool {
    attr.path().is_ident("roxygen")
}
