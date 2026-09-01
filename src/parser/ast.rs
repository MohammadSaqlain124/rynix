// The Abstract Syntax Tree (AST) for Rynix.
//
// Expressions PRODUCE a value. Statements DO something (an action).
// They are separate types so the type system keeps them distinct.

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    // A number literal, e.g. 42. Stored as text so the AST stays
    // independent of how integers are represented (i64 vs bignum).
    Number(String),

    // Using a variable's value, e.g. the `x` in `x + 2`.
    Identifier(String),

    // A unary operation: OP operand, e.g. -5.
    Unary {
        op: UnaryOp,
        operand: Box<Expr>,
    },

    // A binary operation: left OP right, e.g. 2 * 3. Box because the type
    // is recursive (a type cannot directly contain itself — that would be
    // infinite size; Box is a fixed-size pointer to the heap).
    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
    },
}

// A statement performs an action. A program is a sequence of these.
#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    // `let name = value` — DECLARE a new variable (create it).
    Let { name: String, value: Expr },

    // `name = value` — REASSIGN an existing variable. Assigning to a
    // variable that was never declared is an error (checked at runtime).
    Assign { name: String, value: Expr },

    // A bare expression on its own, e.g. `y * 10`.
    Expression(Expr),
}

// A whole program is just a list of statements, run in order.
pub type Program = Vec<Stmt>;

// The unary (prefix) operators. Just negation for now.
#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Negate, // -
}

// The four arithmetic operators. A separate small enum (rather than reusing
// TokenKind) so an operator is always one of these four valid choices.
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
        let expr = Expr::Binary {
            left: Box::new(Expr::Number("1".to_string())),
            op: BinaryOp::Add,
            right: Box::new(Expr::Number("2".to_string())),
        };

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
        let expr = Expr::Binary {
            left: Box::new(Expr::Number("1".to_string())),
            op: BinaryOp::Add,
            right: Box::new(Expr::Binary {
                left: Box::new(Expr::Number("2".to_string())),
                op: BinaryOp::Multiply,
                right: Box::new(Expr::Number("3".to_string())),
            }),
        };

        if let Expr::Binary { op, right, .. } = expr {
            assert_eq!(op, BinaryOp::Add);
            assert!(matches!(*right, Expr::Binary { op: BinaryOp::Multiply, .. }));
        } else {
            panic!("expected a Binary expression at the top");
        }
    }

    #[test]
    fn build_negation_by_hand() {
        let expr = Expr::Unary {
            op: UnaryOp::Negate,
            operand: Box::new(Expr::Number("5".to_string())),
        };

        if let Expr::Unary { op, operand } = expr {
            assert_eq!(op, UnaryOp::Negate);
            assert_eq!(*operand, Expr::Number("5".to_string()));
        } else {
            panic!("expected a Unary expression");
        }
    }

    #[test]
    fn build_let_statement_by_hand() {
        let stmt = Stmt::Let {
            name: "x".to_string(),
            value: Expr::Number("5".to_string()),
        };

        if let Stmt::Let { name, value } = stmt {
            assert_eq!(name, "x");
            assert_eq!(value, Expr::Number("5".to_string()));
        } else {
            panic!("expected a Let statement");
        }
    }

    #[test]
    fn build_assign_statement_by_hand() {
        let stmt = Stmt::Assign {
            name: "x".to_string(),
            value: Expr::Number("10".to_string()),
        };

        if let Stmt::Assign { name, value } = stmt {
            assert_eq!(name, "x");
            assert_eq!(value, Expr::Number("10".to_string()));
        } else {
            panic!("expected an Assign statement");
        }
    }
}