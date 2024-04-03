use proc_macro::{self, TokenStream};

use std::{borrow::BorrowMut, collections::{HashSet, VecDeque}};
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote, quote_spanned};
use syn::{parse_macro_input, spanned::Spanned, Data, DataEnum, DeriveInput, Ident, Token, Variant};

use crate::utils::generate::{generate_field_clones, generate_field_fills, generate_field_idents};

mod utils;


type State = Box<BiplateDeriveState>;

struct BiplateDeriveState {
    /// The type we are deriving Biplate on.
    pub from_ident: syn::Path,

    /// The syn::Data object (struct, enum, or union)
    pub data: Data,

    /// The input tokens.
    /// For use when other fields in this struct don't do the job!
    pub input: DeriveInput,

    /// The current To type.
    pub to_ident: Option<syn::Path>,

    /// Instances of Biplate<To> left to generate.
    pub tos_left: VecDeque<syn::Path>,

    /// All valid biplatable types inside this one.
    pub tos: Vec<syn::Path>

}

impl BiplateDeriveState {
    pub fn new(input: DeriveInput) -> Self {
        BiplateDeriveState { 
            from_ident: input.ident.clone().into(), 
            data: input.data.clone(),
            input, 
            to_ident: None, 
            tos_left: VecDeque::new(),
            tos: Vec::new(),
        }
    }
}

#[proc_macro_derive(Biplate)]
pub fn derive_biplate(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let state: Box<BiplateDeriveState> = Box::new(BiplateDeriveState::new(input));

    derive_biplate_inner(state).unwrap_or_else(syn::Error::into_compile_error).into()
}


/// Wrapper of the main macro function to allow use of Results.
fn derive_biplate_inner(mut state: State) -> syn::Result<TokenStream2> {

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

/// Derives a single Biplate<T> instance from the given state.
fn derive_a_biplate(state: &mut Box<BiplateDeriveState>) -> syn::Result<TokenStream2> {
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
            Data::Struct(x) => derive_biplate_struct(x,state),
            Data::Enum(x) => derive_biplate_enum(x,state),
            Data::Union(x) => derive_biplate_union(x,state),
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


/// The inside of the biplate function for structs.
fn derive_biplate_struct(data: syn::DataStruct,_: &mut Box<BiplateDeriveState>) -> syn::Result<TokenStream2> {
    Err(syn::Error::new_spanned(data.struct_token, "Biplate not yet implemented for structs"))
}

/// The inside of the biplate function for enums.
fn derive_biplate_enum(data: syn::DataEnum, state: &mut Box<BiplateDeriveState>) -> syn::Result<TokenStream2> {

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

fn derive_biplate_enum_variant(variant: syn::Variant,state: &mut Box<BiplateDeriveState>) -> syn::Result<TokenStream2> {
    //Err(syn::Error::new_spanned(variant, format!("TODO: tos to do {:#?}",state.tos)))
    let ident = variant.ident.clone();
    let mut new_idents: Vec<Ident> = Vec::new(); // named _f0, _f1,...
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
fn field_get_tree(idx: usize,field: &syn::Field, state: &mut State) -> syn::Result<TokenStream2>{
    let tree_ident = format_ident!("_f{}_tree",idx);
    let field_ident = format_ident!("_f{}",idx);
    let to = state.to_ident.clone().expect("");
    let curr_ty = field.ty.clone();
    Ok(quote_spanned!{field.span()=>
        let #tree_ident = <#curr_ty as ::uniplate::Biplate<#to>>::biplate(&#field_ident);
    })
}

/// Returns a list of identifiers of to inner types that we can generate a biplate for.
fn get_biplatable_types(data: syn::Data, state: &State) -> syn::Result<Vec<syn::Path>>{
    match data {
        Data::Struct(x) => get_struct_biplatable_types(x,state),
        Data::Enum(x) => get_enum_biplatable_types(x,state),
        Data::Union(x) => get_union_biplatable_types(x,state)
    }
}


fn variant_child_tree(field_idents: &Vec<Ident>) -> syn::Result<TokenStream2> {

    let n = field_idents.len();
    Ok(match n {
        0 => quote!{::uniplate::Tree::Zero},
        1 => quote!{::uniplate::Tree::One(_f0)},
        _ => quote!{::uniplate::Tree::Many(::im::vector![#(#field_idents),*])},
    })
}

fn get_enum_biplatable_types(data: syn::DataEnum, state: &State) -> syn::Result<Vec<syn::Path>>{
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

/// The inside of the biplate function for unions.
fn derive_biplate_union(data: syn::DataUnion, _: &mut Box<BiplateDeriveState>) -> syn::Result<TokenStream2> {
    Err(syn::Error::new_spanned(data.union_token, "Biplate not yet implemented for unions"))
}

fn get_struct_biplatable_types(data: syn::DataStruct, state: &State) -> syn::Result<Vec<syn::Path>>{
    Err(syn::Error::new_spanned(data.struct_token, "TODO"))
}

fn get_union_biplatable_types(data: syn::DataUnion, state: &State) -> syn::Result<Vec<syn::Path>>{
    Err(syn::Error::new_spanned(data.union_token, "TODO"))
}
