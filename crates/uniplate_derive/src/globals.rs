use std::{cell::{Cell, OnceCell, RefCell}, collections::{HashMap, VecDeque}};
use crate::ast::Data;


pub type StatePtr = Box<State>;

/// State variables for biplate derive.
pub struct State {
    /// The type we are deriving Biplate on.
    pub from_ident: syn::Path,

    /// The syn::Data object (struct, enum, or union)
    syn_data: syn::Data,

    /// The uniplate_derive::ast::Data object
    /// Memoised for efficiency (but not with a OnceCell as the operation to fill this is fallable)
    uniplate_data: RefCell<Option<Data>>,

    /// The input tokens.
    /// For use when other fields in this struct don't do the job!
    pub input: syn::DeriveInput,

    /// The current To type.
    pub to_ident: Option<syn::Path>,

    /// Instances of Biplate<To> left to generate.
    pub tos_left: VecDeque<syn::Path>,

    /// All valid biplatable types inside this one.
    pub tos: Vec<syn::Path>,
}

impl State {
    pub fn new(input: syn::DeriveInput) -> Self {
        State { 
            from_ident: input.ident.clone().into(), 
            syn_data: input.data.clone(),
            uniplate_data: RefCell::new(None),
            input, 
            to_ident: None, 
            tos_left: VecDeque::new(),
            tos: Vec::new(),
        }
    }

    /// Gets the uniplate_derive::ast::Data representation of the data structure. 
    ///
    /// The first invokation of this function will parse the data structure to generate this.
    pub fn get_data(&self) -> syn::Result<Data> {
        let data = self.uniplate_data.borrow().clone();
        if let Some(data)  = data  {
            Ok(data.clone())
        } else {
            let data = Data::parse_from_syn(&self.syn_data)?;
            self.uniplate_data.replace(Some(data));
            Ok(data)
        }
    }
}
