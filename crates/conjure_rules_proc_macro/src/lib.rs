//! This is the backend procedural macro crate for `conjure_rules`. USE THAT INSTEAD!

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Ident, ItemFn};

#[proc_macro_attribute]
pub fn register_rule(_: TokenStream, item: TokenStream) -> TokenStream {
    let func = parse_macro_input!(item as ItemFn);
    let rule_ident = &func.sig.ident;
    let static_name = format!("CONJURE_GEN_RULE_{}", rule_ident).to_uppercase();
    let static_ident = Ident::new(&static_name, rule_ident.span());

    let expanded = quote! {
        #func

        #[::conjure_rules::_dependencies::distributed_slice(::conjure_rules::RULES_DISTRIBUTED_SLICE)]
        pub static #static_ident: ::conjure_rules::_dependencies::Rule = ::conjure_rules::_dependencies::Rule {
            name: stringify!(#rule_ident),
            application: #rule_ident,
        };
    };

    TokenStream::from(expanded)
}