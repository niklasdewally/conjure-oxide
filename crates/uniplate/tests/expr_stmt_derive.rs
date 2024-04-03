// Expr and Stmt from the paper, derived using the macro

use std::iter::zip;

use im::vector;
//use uniplate::test_common::paper::*;
use uniplate::biplate::*;
use uniplate::Biplate;
use uniplate::Tree;

use self::Expr::*;

// Stmt and Expr to demonstrate and test multitype traversals.
#[derive(Eq, PartialEq, Clone, Debug, Biplate)]
pub enum Stmt {
    Assign(String, Expr),
    Sequence(Vec<Stmt>),
    If(Expr, Box<Stmt>, Box<Stmt>),
    While(Expr, Box<Stmt>),
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum Expr {
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Val(i32),
    Var(String),
    Neg(Box<Expr>),
}

use Stmt::*;
use Tree::*;

#[test]
fn children_bi_multitype() {
    let my_stmt = Sequence(vec![
        While(
            Val(0),
            Box::new(Assign(
                "x".to_owned(),
                Add(Box::new(Var("x".to_owned())), Box::new(Val(10))),
            )),
        ),
        If(
            Var("x".to_string()),
            Box::new(Assign(
                "x".to_string(),
                Add(Box::new(Var("x".to_owned())), Box::new(Val(10))),
            )),
            Box::new(Sequence(vec![])),
        ),
    ]);

    let expected_expr_children = 4;

    let children = <Stmt as Biplate<Expr>>::children_bi(&my_stmt);

    assert_eq!(expected_expr_children, children.len());

    println!("{:?}", children);
    let Val(_) = children[0] else { panic!() };
    let Add(_, _) = children[1] else { panic!() };
    let Var(_) = children[2] else { panic!() };
    let Add(_, _) = children[3] else { panic!() };
}

#[test]
fn universe_bi_multitype() {
    let my_stmt = Sequence(vec![
        While(
            Val(0),
            Box::new(Assign(
                "x".to_owned(),
                Add(Box::new(Var("x".to_owned())), Box::new(Val(10))),
            )),
        ),
        If(
            Var("x".to_string()),
            Box::new(Assign(
                "x".to_string(),
                Add(Box::new(Var("x".to_owned())), Box::new(Val(10))),
            )),
            Box::new(Sequence(vec![])),
        ),
    ]);

    let expected_expr_universe = 8;

    let children = <Stmt as Biplate<Expr>>::universe_bi(&my_stmt);

    assert_eq!(expected_expr_universe, children.len());

    println!("{:?}", children);
    let Val(_) = children[0] else { panic!() };
    let Add(_, _) = children[1] else { panic!() };
    let Var(_) = children[2] else { panic!() };
    let Val(_) = children[3] else { panic!() };
    let Var(_) = children[4] else { panic!() };
    let Add(_, _) = children[5] else { panic!() };
    let Var(_) = children[6] else { panic!() };
    let Val(_) = children[7] else { panic!() };
}
