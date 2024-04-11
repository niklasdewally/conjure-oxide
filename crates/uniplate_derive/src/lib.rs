mod utils;
mod ast;
mod globals;
mod enums;
mod unions;
mod structs;

use globals::{State, StatePtr};
use proc_macro::{self, TokenStream};

use std::borrow::BorrowMut;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Data, DeriveInput,parse_macro_input};

use crate::structs::*;
use crate::enums::*;
use crate::unions::*;


#[proc_macro_derive(Biplate)]
pub fn derive_biplates(input: TokenStream) -> TokenStream {

    let input = parse_macro_input!(input as DeriveInput);
    let state: StatePtr = StatePtr::new(State::new(input));

    fn _inner(mut state: StatePtr) -> syn::Result<TokenStream2> {
    
        // What types can we do biplate on?
        state.tos.extend(get_biplatable_types(state.data.clone(),&state)?);
    
        // We need to generate Biplate instances for all of those types.
        state.tos_left.extend(state.tos.clone().into_iter());
    
        let mut out_tokens: Vec<TokenStream2> = vec![];
        while let Some(to_id)  = state.tos_left.pop_front() {
            state.to_ident = Some(to_id);
            out_tokens.push(derive_a_biplate(state.borrow_mut())?);
        }
        Ok(out_tokens.into_iter().collect())
    }

    _inner(state).unwrap_or_else(syn::Error::into_compile_error).into()
}



/// Derives a single Biplate<T> instance from the given state.
fn derive_a_biplate(state: &mut StatePtr) -> syn::Result<TokenStream2> {
    #[allow(clippy::expect_used)]
    let to_ident = state.to_ident.clone().expect("");
    let from_ident = state.from_ident.clone();

    let biplate_impl = if state.to_ident.clone().expect("") == state.from_ident{
        // Identity biplate
        quote!{
            (::uniplate::Tree::One(self.clone()),Box::new(|t| {
                let One(x) = t else {panic!() };
                x
            }))}
    } else {
        match state.data.clone() {
            Data::Struct(x) => biplate_impl_for_struct(x,state),
            Data::Enum(x) => biplate_impl_for_gnum(x,state),
            Data::Union(x) => biplate_impl_for_union(x,state),
        }?
    };

    Ok(quote! {
        impl ::uniplate::Biplate<#to_ident> for #from_ident {
            fn biplate(&self) -> (::uniplate::Tree<#to_ident>,Box<dyn Fn(::uniplate::Tree<#to_ident>) -> #from_ident>) {
                #biplate_impl
            }
        }
    })
}



/// Returns a list of identifiers of to inner types that we can generate a biplate for.
fn get_biplatable_types(data: syn::Data, state: &StatePtr) -> syn::Result<Vec<syn::Path>>{
    match data {
        Data::Struct(x) => get_struct_biplatable_types(x,state),
        Data::Enum(x) => get_enum_biplatable_types(x,state),
        Data::Union(x) => get_union_biplatable_types(x,state)
    }
}
