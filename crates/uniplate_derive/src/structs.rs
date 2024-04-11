//! Biplate functions for structs

use proc_macro2::TokenStream as TokenStream2;
use crate::globals::*;

/// The inside of the biplate function for structs.
pub fn biplate_impl_for_struct(data: syn::DataStruct,_: &mut StatePtr) -> syn::Result<TokenStream2> {
    Err(syn::Error::new_spanned(data.struct_token, "Biplate not yet implemented for structs"))
}

pub fn get_struct_biplatable_types(data: syn::DataStruct, state: &StatePtr) -> syn::Result<Vec<syn::Path>>{
    Err(syn::Error::new_spanned(data.struct_token, "TODO"))
}
