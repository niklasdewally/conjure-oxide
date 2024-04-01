use proc_macro::{self, TokenStream};

use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DataEnum, DeriveInput, Ident, Variant};

use crate::utils::generate::{generate_field_clones, generate_field_fills, generate_field_idents};

mod utils;

#[proc_macro_derive(Biplate)]
pub fn derive_biplate(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let from_ident = &input.ident;
    let to_ident = &input.generics.type_params().next();
    let data = &input.data;

    // TODO: get from and to types.
    match data {
        Data::Union(data) => Err(syn::Error::new_spanned(
            input,
            "Biplate not implemented for unions",
        )),
        Data::Struct(data) => Err(syn::Error::new_spanned(
            input,
            "Biplate not implemented for structs",
        )),
        Data::Enum(data) => Err(syn::Error::new_spanned(
            input,
            "Biplate not implemented for enums",
        )),
    }
    .unwrap_or_else(syn::Error::into_compile_error)
    .into()
}
