#![doc= include_str!("../Readme.md")]
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Attribute, FnArg, Ident, ItemFn, Pat};
use util::{extract_doc_attrs, extract_fn_doc_attrs, prepend_to_doc_attribute};
mod util;

#[proc_macro_attribute]
/// the principal attribute inside this crate that lets us document function arguments
pub fn doxidize(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut function: ItemFn = parse_macro_input!(item as ItemFn);

    // will contain the docs comments for each documented function parameter
    // together with the identifier of the function parameter.
    let mut parameter_docs =
        Vec::<(&Ident, Vec<Attribute>)>::with_capacity(function.sig.inputs.len());

    // extrac the doc attributes on the function itself
    let function_docs = match extract_fn_doc_attrs(&mut function.attrs) {
        Ok(docs) => docs,
        Err(err) => {
            return err.into_compile_error().into();
        }
    };
    // extract the doc attributes on the parameters
    for arg in function.sig.inputs.iter_mut() {
        match arg {
            FnArg::Typed(pat_type) => {
                let Pat::Ident(pat_ident) = pat_type.pat.as_ref() else {
                    unreachable!("unexpected node while parsing");
                };
                let ident = &pat_ident.ident;
                let docs = extract_doc_attrs(&mut pat_type.attrs);

                if !docs.is_empty() {
                    parameter_docs.push((ident, docs));
                }
            }
            FnArg::Receiver(_) => {}
        }
    }

    if parameter_docs.is_empty() {
        return syn::Error::new_spanned(
            function.sig.ident,
            "Function has no documented arguments.\nDocument at least one function argument.",
        )
        .into_compile_error()
        .into();
    }

    let parameter_doc_blocks = parameter_docs.into_iter().map(|(ident, docs)| {
        let mut docs_iter = docs.iter();
        // we always have at least one doc attribute because otherwise we
        // would not have inserted this pair into the parameter docs in the
        // first place
        let first = docs_iter
            .next()
            .expect("unexpectedly encountered empty doc list");

        let first_line = prepend_to_doc_attribute(&format!(" * `{}`:", ident), first);

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
        #[doc="**Arguments**: "]
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
// to make sure that this attribute is not placed before the #[doxidize]
// attribute. All other logic is handled in the doxidize macro itself.
/// a helper attribute that dictates the placement of the section documenting
/// the function arguments
#[proc_macro_attribute]
pub fn arguments_section(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let function: ItemFn = parse_macro_input!(item as ItemFn);

    // enforce that this macro comes after doxidize, which means it
    // cannot see the doxidize attribute
    let maybe_doxidize = function.attrs.iter().find(|attr| is_doxidize_main(attr));
    if let Some(attr) = maybe_doxidize {
        syn::Error::new_spanned(&attr,"The #[doxidize] attribute must come before the arguments section attribute.\nPlace it before any of the doc comments for the function.").into_compile_error().into()
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

/// check whether an attribute is the raw #[doxidize] main attribute.
/// Stuck into this function, so I can refactor this logic
#[inline(always)]
fn is_doxidize_main(attr: &Attribute) -> bool {
    attr.path().is_ident("doxidize")
}
