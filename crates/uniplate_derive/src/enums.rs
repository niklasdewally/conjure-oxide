//! Biplate functions for enums

use std::collections::HashSet;

use proc_macro2::TokenStream as TokenStream2;
use syn::spanned::Spanned as _;
use crate::globals::*;
use quote::{format_ident, quote, quote_spanned};

/// The inside of the biplate function for enums.
pub fn biplate_impl_for_enum(data: syn::DataEnum, state: &mut StatePtr) -> syn::Result<TokenStream2> {

    let mut variant_subsitutions: Vec<TokenStream2> = vec![];

    for variant in data.variants {
        variant_subsitutions.push(derive_biplate_enum_variant(variant,state)?);
    }

    Ok(quote!{
        match (self) {
            #(#variant_subsitutions)*
            }
    })
}

fn derive_biplate_enum_variant(variant: syn::Variant,state: &mut StatePtr) -> syn::Result<TokenStream2> {
    //Err(syn::Error::new_spanned(variant, format!("TODO: tos to do {:#?}",state.tos)))
    let ident = variant.ident.clone();
    let mut new_idents: Vec<syn::Ident> = Vec::new(); // named _f0, _f1,...
    let mut new_trees: Vec<TokenStream2> = Vec::new();
    for (i,field) in variant.fields.iter().enumerate() {
        new_idents.push(format_ident!("_f{}",i));
        new_trees.push(field_get_tree(i,field,state)?);
    }

    let child_tree = variant_child_tree(&new_idents)?;

    Ok(quote_spanned! {ident.span()=>
        #ident(#(#new_idents),*) => {
            #(#new_trees)*
            (#child_tree,todo!())
        },
    })
}

/// Generate the Tree node for the given enum field
fn field_get_tree(idx: usize,field: &syn::Field, state: &mut StatePtr) -> syn::Result<TokenStream2>{
    let tree_ident = format_ident!("_f{}_tree",idx);
    let field_ident = format_ident!("_f{}",idx);
    let to = state.to_ident.clone().expect("");
    let curr_ty = field.ty.clone();
    Ok(quote_spanned!{field.span()=>
        let #tree_ident = <#curr_ty as ::uniplate::Biplate<#to>>::biplate(&#field_ident);
    })
}

fn variant_child_tree(field_idents: &Vec<syn::Ident>) -> syn::Result<TokenStream2> {

    let n = field_idents.len();
    Ok(match n {
        0 => quote!{::uniplate::Tree::Zero},
        1 => quote!{::uniplate::Tree::One(_f0)},
        _ => quote!{::uniplate::Tree::Many(::im::vector![#(#field_idents),*])},
    })
}

pub fn get_enum_biplatable_types(data: syn::DataEnum, state: &StatePtr) -> syn::Result<Vec<syn::Path>>{
    // use set for dedup.
    let mut set: HashSet<syn::Path> = HashSet::new();
    for variant in data.variants {
        for field in variant.fields {
            let type_path: syn::Path = parse_type(field.ty)?;
            set.insert(type_path);
        }
    }
    Ok(set.into_iter().collect())
}

/// Parses a Type value, returning the unboxed inner type.
fn parse_type(ty: syn::Type) -> syn::Result<syn::Path> {

    match ty{
        syn::Type::Path(path) => parse_typepath(path.path),
        _ => Err(syn::Error::new_spanned(ty.clone(),"unsupported type!"))
        //syn::Type::BareFn(_) => todo!(),
        //syn::Type::Group(_) => todo!(),
        //syn::Type::ImplTrait(_) => todo!(),
        //syn::Type::Infer(_) => todo!(),
        //syn::Type::Macro(_) => todo!(),
        //syn::Type::Never(_) => todo!(),
        //syn::Type::Paren(_) => todo!(),
        //syn::Type::Ptr(_) => todo!(),
        //syn::Type::Reference(_) => todo!(),
        //syn::Type::Slice(_) => todo!(),
        //syn::Type::TraitObject(_) => todo!(),
        //syn::Type::Tuple(_) => todo!(), TODO: implement
        //syn::Type::Verbatim(_) => todo!(),
        //syn::Type::Array(_) => todo!(),
    }
}

/// Parses a Type::Path value, returning the unboxed inner type.
fn parse_typepath(path: syn::Path) -> syn::Result<syn::Path>{
    // Jump into angle brackets etc until we just find a normal type.
    let segments = path.segments.clone();
    let first_segment = path.segments.first().expect("").clone();
    let seg_ident = first_segment.ident.clone();

    // Peek at first, if it doenst have any <> we are done.
    // Otherwise, go into the <> and find the next biggest path and process that.
    match (first_segment.arguments) {
        syn::PathArguments::None => return Ok(path),
        syn::PathArguments::Parenthesized(x) => return Err(syn::Error::new_spanned(x,"biplate: unsupported type")),
        syn::PathArguments::AngleBracketed(args) => {
            if seg_ident != "Box" && seg_ident != "Vec" {
                return Err(syn::Error::new_spanned(path,format!("biplate: unsupported type: {:}",seg_ident)));
            } 
            // Boxes only have a single generic type parameter, and we expect the thing we find to
            // be a proper type.
            
            // TODO: could use parse_quote instead of all this?
            if let syn::GenericArgument::Type(ty2) = args.args[0].clone() {
                return parse_type(ty2)
            } else{
                return Err(syn::Error::new_spanned(path.clone(),"biplate: unsupported type"))
            }
        }
    }
}
