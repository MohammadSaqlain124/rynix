// The Abstract Syntax Tree (AST) for Rynix expressions.
//
// An expression is one of a few shapes, and some shapes contain other
// expressions — that recursion is what lets expressions nest to any
// depth (e.g. 1 + 2 * 3 - 4).

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    // A number literal, e.g. 42.
    // Stored as text for the same reason as in the lexer: the AST should
    // not commit to how integers are represented (i64 vs bignum). A later
    // phase interprets the digits.
    Number(String),

    // A binary operation: left OP right, e.g. 2 * 3.
    // The left and right sides are themselves expressions, so this is the
    // recursive case. We use Box because a type cannot directly contain
    // itself — that would need infinite size. Box stores a pointer (fixed,
    // known size) to an Expr living on the heap, which makes the type
    // finite while still allowing arbitrary nesting.
    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
    },
}

// The four arithmetic operators. A separate small enum (rather than reusing
// TokenKind) so the type system guarantees an operator is always one of
// these four valid choices — you can't accidentally build a Binary with,
// say, a parenthesis as its operator.
#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    Add,      // +
    Subtract, // -
    Multiply, // *
    Divide,   // /
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number_expression() {
        let expr = Expr::Number("42".to_string());
        assert_eq!(expr, Expr::Number("42".to_string()));
    }

    #[test]
    fn build_one_plus_two_by_hand() {
        // Represents: 1 + 2
        let expr = Expr::Binary {
            left: Box::new(Expr::Number("1".to_string())),
            op: BinaryOp::Add,
            right: Box::new(Expr::Number("2".to_string())),
        };

        // Check the shape piece by piece.
        if let Expr::Binary { left, op, right } = expr {
            assert_eq!(*left, Expr::Number("1".to_string()));
            assert_eq!(op, BinaryOp::Add);
            assert_eq!(*right, Expr::Number("2".to_string()));
        } else {
            panic!("expected a Binary expression");
        }
    }

    #[test]
    fn build_one_plus_two_times_three_by_hand() {
        // Represents: 1 + 2 * 3, correctly shaped so * is lower than +.
        //
        //        +
        //       / \
        //      1   *
        //         / \
        //        2   3
        let expr = Expr::Binary {
            left: Box::new(Expr::Number("1".to_string())),
            op: BinaryOp::Add,
            right: Box::new(Expr::Binary {
                left: Box::new(Expr::Number("2".to_string())),
                op: BinaryOp::Multiply,
                right: Box::new(Expr::Number("3".to_string())),
            }),
        };

        // The top node must be a `+` whose right child is a `*`.
        if let Expr::Binary { op, right, .. } = expr {
            assert_eq!(op, BinaryOp::Add);
            assert!(matches!(*right, Expr::Binary { op: BinaryOp::Multiply, .. }));
        } else {
            panic!("expected a Binary expression at the top");
        }
    }
}