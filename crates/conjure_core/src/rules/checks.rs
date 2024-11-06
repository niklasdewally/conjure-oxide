use uniplate::Uniplate;

use crate::ast::Expression;

/// True iff the entire AST is constants.
pub fn is_all_constant(expression: &Expression) -> bool {
    for expr in expression.universe() {
        match expr {
            Expression::Constant(_, _) => {}
            Expression::Reference(_, _) => {
                return false;
            }
            _ => {}
        }
    }

    true
}

/// True iff the expression is a constant or a reference.
pub fn is_leaf(expression: &Expression) -> bool {
    matches!(
        expression,
        Expression::Constant(_, _) | Expression::Reference(_, _)
    )
}

/// True iff the expression contains no nested sub-expressions.
pub fn is_flat(expr: &Expression) -> bool {
    for e in expr.children() {
        if !is_leaf(&e) {
            return false;
        }
    }
    true
}
