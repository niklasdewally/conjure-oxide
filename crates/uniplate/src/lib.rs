//! A port of Haskell's [Uniplate](https://hackage.haskell.org/package/uniplate) in Rust.
//!
//!
//! # Examples
//!
//! ## A Calculator Input Language
//!
//! Consider the AST of a calculator input language:
//!
//! ```
//! pub enum AST {
//!     Int(i32),
//!     Add(Box<AST>,Box<AST>),
//!     Sub(Box<AST>,Box<AST>),
//!     Div(Box<AST>,Box<AST>),
//!     Mul(Box<AST>,Box<AST>)
//! }
//!```
//!
//! Using uniplate, one can implement a single function for this AST that can be used in a whole
//! range of traversals.
//!
//! While this does not seem helpful in this toy example, the benefits amplify when the number of
//! enum variants increase, and the different types contained in their fields increase.
//!
//!
//! The below example implements [`Uniplate`](uniplate::Uniplate) for this language AST, and uses uniplate methods to
//! evaluate the encoded equation.
//!
//!```
//! use uniplate::uniplate::{Uniplate, UniplateError};
//!
//! #[derive(Clone,Eq,PartialEq,Debug)]
//! pub enum AST {
//!     Int(i32),
//!     Add(Box<AST>,Box<AST>),
//!     Sub(Box<AST>,Box<AST>),
//!     Div(Box<AST>,Box<AST>),
//!     Mul(Box<AST>,Box<AST>)
//! }
//!
//! // In the future would be automatically derived.
//! impl Uniplate for AST {
//!     fn uniplate(&self) -> (Vec<AST>, Box<dyn Fn(Vec<AST>) -> Result<AST, UniplateError> +'_>) {
//!         let context: Box<dyn Fn(Vec<AST>) -> Result<AST, UniplateError>> = match self {
//!             AST::Int(i) =>    Box::new(|_| Ok(AST::Int(*i))),
//!             AST::Add(_, _) => Box::new(|exprs: Vec<AST>| Ok(AST::Add(Box::new(exprs[0].clone()),Box::new(exprs[1].clone())))),
//!             AST::Sub(_, _) => Box::new(|exprs: Vec<AST>| Ok(AST::Sub(Box::new(exprs[0].clone()),Box::new(exprs[1].clone())))),
//!             AST::Div(_, _) => Box::new(|exprs: Vec<AST>| Ok(AST::Div(Box::new(exprs[0].clone()),Box::new(exprs[1].clone())))),
//!             AST::Mul(_, _) => Box::new(|exprs: Vec<AST>| Ok(AST::Mul(Box::new(exprs[0].clone()),Box::new(exprs[1].clone()))))
//!         };
//!
//!         let children: Vec<AST> = match self {
//!             AST::Add(a,b) => vec![*a.clone(),*b.clone()],
//!             AST::Sub(a,b) => vec![*a.clone(),*b.clone()],
//!             AST::Div(a,b) => vec![*a.clone(),*b.clone()],
//!             AST::Mul(a,b) => vec![*a.clone(),*b.clone()],
//!             _ => vec![]
//!         };
//!
//!         (children,context)
//!     }
//! }
//!
//! pub fn my_rule(e: AST) -> AST{
//!     match e {
//!         AST::Int(a) => AST::Int(a),
//!         AST::Add(a,b) => {match (&*a,&*b) { (AST::Int(a), AST::Int(b)) => AST::Int(a+b), _ => AST::Add(a,b) }}
//!         AST::Sub(a,b) => {match (&*a,&*b) { (AST::Int(a), AST::Int(b)) => AST::Int(a-b), _ => AST::Sub(a,b) }}
//!         AST::Mul(a,b) => {match (&*a,&*b) { (AST::Int(a), AST::Int(b)) => AST::Int(a*b), _ => AST::Mul(a,b) }}
//!         AST::Div(a,b) => {match (&*a,&*b) { (AST::Int(a), AST::Int(b)) => AST::Int(a/b), _ => AST::Div(a,b) }}
//!     }
//! }
//! pub fn main() {
//!     let ast = AST::Add(
//!                 Box::new(AST::Int(1)),
//!                 Box::new(AST::Mul(
//!                     Box::new(AST::Int(2)),
//!                     Box::new(AST::Div(
//!                         Box::new(AST::Add(Box::new(AST::Int(1)),Box::new(AST::Int(2)))),
//!                         Box::new(AST::Int(3))
//!                     )))));
//!
//!     let new_ast = ast.transform(my_rule);
//!     assert!(new_ast.is_ok());
//!     println!("{:?}",new_ast);
//!     assert_eq!(new_ast.unwrap(), AST::Int(3));
//! }
//! ```
//!
//! ....MORE DOCS TO COME....
//!
//! # Acknowledgements / Related Work
//!
//! *This crate implements programming constructs from the following Haskell libraries and
//! papers:*
//!  
//! * [Uniplate](https://hackage.haskell.org/package/uniplate).
//!
//! * Neil Mitchell and Colin Runciman. 2007. Uniform boilerplate and list processing. In
//! Proceedings of the ACM SIGPLAN workshop on Haskell workshop (Haskell '07). Association for
//! Computing Machinery, New York, NY, USA, 49–60. <https://doi.org/10.1145/1291201.1291208>
//! [(free copy)](https://www.cs.york.ac.uk/plasma/publications/pdf/MitchellRuncimanHW07.pdf)
//!
//! * Huet G. The Zipper. Journal of Functional Programming. 1997;7(5):549–54. <https://doi.org/10.1017/S0956796897002864>
//! [(free copy)](https://www.cambridge.org/core/services/aop-cambridge-core/content/view/0C058890B8A9B588F26E6D68CF0CE204/S0956796897002864a.pdf/zipper.pdf)

pub mod biplate;
pub use biplate::*;
mod tree;
//pub mod uniplate;

pub use tree::Tree;

#[doc(hidden)]
pub mod test_common;

extern crate self as uniplate;
pub use uniplate_derive::*;
