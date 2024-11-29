use proc_macro2::TokenStream;
use quote::quote;
use syn::{Attribute, Expr, FnArg, Generics, Ident, LitStr, Meta, MetaNameValue, Pat};

use crate::is_parameters_section;

/// Function to prepend a string to a `#[doc]` attribute.
pub fn prepend_to_doc_attribute(prepend_text: &str, attr: &Attribute) -> proc_macro2::TokenStream {
    // Parse the attribute to see if it's a MetaNameValue (e.g., #[doc = "..."])
    assert!(
        attr.path().is_ident("doc"),
        "function must only be called on doc attributes"
    );
    if let Meta::NameValue(MetaNameValue {
        value: Expr::Lit(ref lit),
        ..
    }) = attr.meta
    {
        let syn::ExprLit {
            attrs: _,
            lit: syn::Lit::Str(doc_string),
        } = lit
        else {
            unreachable!("reached unexpected node while parsing");
        };
        // Prepend the text to the existing doc comment
        let new_doc = format!("{}{}", prepend_text, doc_string.value());

        // Create a new string literal with the modified doc string
        let new_doc_lit = LitStr::new(&new_doc, doc_string.span());

        // Create a new attribute with the modified doc string (enclosed in quotes)
        let new_attr = quote! {
            #[doc = #new_doc_lit]
        };
        new_attr
    } else {
        unreachable!("reached unexpected node while parsing");
    }
}

/// removes the #[doc...] attributes from `attrs` and returns them in
/// it's own vector.
pub fn extract_doc_attrs(attrs: &mut Vec<Attribute>) -> Vec<Attribute> {
    let doc_attrs = attrs
        .iter()
        .filter(|attr| attr.path().is_ident("doc"))
        .cloned()
        .collect();
    attrs.retain(|attr| !attr.path().is_ident("doc"));
    doc_attrs
}

/// function doc attributes split by whether they occur above or below
/// the attribute that indicates where the argument section should be placed.
/// If no such attribute is present, we stick all the doc attributes in the
/// before section
pub struct FunctionDocs {
    pub before_args_section: Vec<Attribute>,
    pub after_args_section: Vec<Attribute>,
}

/// extract the documentation from the doc comments of the function and perform
/// some additional logic
pub fn extract_fn_doc_attrs(attrs: &mut Vec<Attribute>) -> Result<FunctionDocs, syn::Error> {
    let mut before_args_section = Vec::with_capacity(attrs.len());
    let mut after_args_section = Vec::with_capacity(attrs.len());

    // I'm sure this could be done with iterators...
    // I'm just too dumb to do that now
    let mut idx = 0;
    // parse the arguments before the arguments-section attribute
    while idx < attrs.len() {
        let current_attr = attrs.get(idx).unwrap();
        if is_parameters_section(current_attr) {
            idx += 1;
            break;
        }
        if current_attr.path().is_ident("doc") {
            before_args_section.push(current_attr.clone());
        }
        idx += 1;
    }

    while idx < attrs.len() {
        let current_attr = attrs.get(idx).unwrap();
        if is_parameters_section(current_attr) {
            return Err(syn::Error::new_spanned(
                current_attr,
                "Duplicate attribute not allowed.",
            ));
        }
        if current_attr.path().is_ident("doc") {
            after_args_section.push(current_attr.clone());
        }
        idx += 1;
    }

    // delete all doc attributes from the function (and the arguments section attributes that I don't need anymore)
    attrs.retain(|attr| !attr.path().is_ident("doc"));
    Ok(FunctionDocs {
        before_args_section,
        after_args_section,
    })
}

/// an identifier (such as a function parameter or a generic type)
/// with doc attributes
pub struct DocumentedIdent<'a> {
    pub ident: &'a Ident,
    /// the doc comments
    pub docs: Vec<Attribute>,
}

impl<'a> DocumentedIdent<'a> {
    pub fn new(ident: &'a Ident, docs: Vec<Attribute>) -> Self {
        Self { ident, docs }
    }
}

/// extract the parameter documentation from an iterator over function arguments.
/// This will also remove all the doc comments from the collection of attributes, but
/// will leave all the other attributes untouched.
pub fn extract_documented_parameters<'a, I>(args: I) -> Result<Vec<DocumentedIdent<'a>>, syn::Error>
where
    I: Iterator<Item = &'a mut FnArg>,
{
    // will contain the docs comments for each documented function parameter
    // together with the identifier of the function parameter.
    let (lower, upper) = args.size_hint();
    let mut documented_params = Vec::<DocumentedIdent>::with_capacity(upper.unwrap_or(lower));

    for arg in args {
        match arg {
            FnArg::Typed(pat_type) => {
                let Pat::Ident(pat_ident) = pat_type.pat.as_ref() else {
                    unreachable!("unexpected node while parsing");
                };
                let ident = &pat_ident.ident;
                let docs = extract_doc_attrs(&mut pat_type.attrs);

                if !docs.is_empty() {
                    documented_params.push(DocumentedIdent::new(ident, docs));
                }
            }
            FnArg::Receiver(_) => {}
        }
    }
    Ok(documented_params)
}

/// extract the documented generic parameters from a collection of generics as
/// a collection of documented identifiers
/// This will also remove all the doc comments from the collection of generics, but
/// will leave all the other attributes untouched.
pub fn extract_documented_generics(
    generics: &'_ mut Generics,
) -> Result<Vec<DocumentedIdent<'_>>, syn::Error> {
    let mut documented_generics = Vec::with_capacity(generics.params.len());
    for param in generics.params.iter_mut() {
        let (ident, attrs) = match param {
            syn::GenericParam::Lifetime(lif) => (&lif.lifetime.ident, &mut lif.attrs),
            syn::GenericParam::Type(ty) => (&ty.ident, &mut ty.attrs),
            syn::GenericParam::Const(con) => (&con.ident, &mut con.attrs),
        };
        let docs = extract_doc_attrs(attrs);
        if !docs.is_empty() {
            documented_generics.push(DocumentedIdent::new(ident, docs))
        }
    }

    Ok(documented_generics)
}

/// make a documentation block, which is a markdown list of
/// **<caption>**
///
/// * `ident`: doc-comments
/// * `ident2`: doc-comments
/// * ...
///
/// returns an empty token stream if the list of idents is empty
pub fn make_doc_block<S>(
    caption: S,
    documented_idents: Vec<DocumentedIdent<'_>>,
) -> Option<TokenStream>
where
    S: AsRef<str>,
{
    let has_documented_idents = !documented_idents.is_empty();

    let list = documented_idents.into_iter().map(|param| {
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

    let caption = format!(" **{}**:", caption.as_ref());

    if has_documented_idents {
        Some(quote! {
        #[doc=""]
        #[doc=#caption]
        #[doc=""]
        #(#list)*
        })
    } else {
        None
    }
}
