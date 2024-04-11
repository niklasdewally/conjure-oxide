//! Parsing types

// useful related SO link (but not directly used here yet)
// https://stackoverflow.com/questions/55271857/how-can-i-get-the-t-from-an-optiont-when-using-syn


use proc_macro2::Span;
use quote::{quote_spanned, ToTokens, TokenStreamExt as _};

/// All valid field wrapper types - e.g Box, Vec, ...
#[derive(Clone)]
pub enum WrapperTypes {
    Box,
    Vec,
    None
}

use WrapperTypes::*;

/// Stores type info for a Biplate field.
///
/// For a type Box<T>, Box is the wrapper type, and T is the underlying type.
///
/// This struct implements ToTokens so can be used in quote! macros.
#[derive(Clone)]
pub struct TypeInfo {
    /// The underlying type of the field.
    pub base_typ: syn::Path,

    /// The wrapper type of the field.
    pub wrapper_typ: WrapperTypes,

    pub span: Span
}

impl ToTokens for TypeInfo {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let ty = self.base_typ.clone();
        let new_tokens = match self.wrapper_typ {
            Box  => quote_spanned!{self.span=>::std::Box<#ty>},
            Vec  => quote_spanned!{self.span=>::std::Vec<#ty>},
            None => quote_spanned!{self.span=>#ty}
        };

        tokens.append_all(new_tokens);
    }
}
