//! Storage wrapper of a struct, enum, or union data structure.
use super::TypeInfo;
use proc_macro2::Span;
use syn::spanned::Spanned as _;
use quote::{quote_spanned, ToTokens, TokenStreamExt as _};

#[derive(Clone)]
pub enum Data {
    DataEnum(DataEnum)
}
impl Data  {
    pub fn parse_from_syn(input: &syn::Data) -> syn::Result<Self>{
        match input {
            syn::Data::Enum(x) => Ok(Data::DataEnum(DataEnum::parse_from_syn(x)?)),
            syn::Data::Struct(x) => Err(syn::Error::new(x.fields.span(),"Biplate implemented for structs.")),
            syn::Data::Union(x) => Err(syn::Error::new(x.fields.span(),"Biplate implemented for unions.")),
        }
    }
}


#[derive(Clone)]
pub struct DataEnum {
    pub ident: syn::Ident,
    pub span:  Span,
    pub variants: Vec<Variant>
}

impl DataEnum {
    pub fn parse_from_syn(input: &syn::DataEnum) -> syn::Result<Self>{
        todo!()
    }
}

#[derive(Clone)]
pub struct Variant {
    pub ident: syn::Ident,
    pub span: Span,
    pub fields: Vec<Field>
}

#[derive(Clone)]
pub struct Field {
    pub ident: syn::Ident,
    pub span: Span,
    pub typ: FieldType
}

#[derive(Clone)]
pub enum FieldType {
    /// This field can be plated.
    Plateable(TypeInfo),

    /// This field cannot be plated.
    Unplateable
}
