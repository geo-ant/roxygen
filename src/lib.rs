use quote::quote;
use syn::{parse_macro_input, Attribute, FnArg, Ident, ItemFn, Pat};
use util::{extract_doc_attrs, prepend_to_doc_attribute};
mod util;

#[proc_macro_attribute]
pub fn doxidize(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut function: ItemFn = parse_macro_input!(item as ItemFn);

    // will contain the docs comments for each documented function parameter
    // together with the identifier of the function parameter.
    let mut parameter_docs =
        Vec::<(&Ident, Vec<Attribute>)>::with_capacity(function.sig.inputs.len());

    for attr in function.attrs.iter() {
        println!("attr: {:?}", attr.path());
    }

    // extrac the doc attributes on the function itself
    let function_docs = extract_doc_attrs(&mut function.attrs);
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
            "function has no documented parameters",
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

    quote! {
        #(#function_docs)*
        #[doc=""]
        #[doc=" # Arguments"]
        #[doc=""]
        #(#parameter_doc_blocks)*
        #function
    }
    .into()
}

// helper attribute
//@todo document
#[proc_macro_attribute]
pub fn arguments(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    todo!("make sure this attribute comes after doxidize")
    //@todo and also make sure that it does not occur multiple times
    item
}
