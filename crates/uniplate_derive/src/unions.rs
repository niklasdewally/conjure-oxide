//! Biplate functions for Unions

use proc_macro2::TokenStream as TokenStream2;
use crate::globals::StatePtr;

/// The inside of the biplate function for unions.
pub fn biplate_impl_for_union(data: syn::DataUnion, _: &mut StatePtr) -> syn::Result<TokenStream2> {
    Err(syn::Error::new_spanned(data.union_token, "Biplate not yet implemented for unions"))
}


pub fn get_union_biplatable_types(data: syn::DataUnion, state: &StatePtr) -> syn::Result<Vec<syn::Path>>{
    Err(syn::Error::new_spanned(data.union_token, "TODO"))
}
