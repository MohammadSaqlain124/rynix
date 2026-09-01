use crate::parser::ast::{BinaryOp, Expr, UnaryOp};

// Evaluate an expression tree down to a single integer.
//
// Rules:
//   - Number  -> interpret its text as an i64.
//   - Unary   -> evaluate the operand, then apply the unary operator.
//   - Binary  -> evaluate both children, then combine with the operator.
//
// The children-first order means precedence (already encoded in the tree
// shape by the parser) is obeyed automatically.
pub fn eval(expr: &Expr) -> Result<i64, String> {
    match expr {
        Expr::Number(text) => text
            .parse::<i64>()
            .map_err(|_| format!("invalid integer literal: {}", text)),

        Expr::Unary { op, operand } => {
            let value = eval(operand)?;
            match op {
                UnaryOp::Negate => Ok(-value),
            }
        }

        Expr::Binary { left, op, right } => {
            let l = eval(left)?;
            let r = eval(right)?;
            match op {
                BinaryOp::Add => Ok(l + r),
                BinaryOp::Subtract => Ok(l - r),
                BinaryOp::Multiply => Ok(l * r),
                BinaryOp::Divide => {
                    if r == 0 {
                        Err("division by zero".to_string())
                    } else {
                        Ok(l / r)
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::lexer::Lexer;
    use crate::parser::parser::Parser;

    fn eval_str(source: &str) -> Result<i64, String> {
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize()?;
        let mut parser = Parser::new(tokens);
        let expr = parser.parse()?;
        eval(&expr)
    }

    #[test]
    fn single_number() {
        assert_eq!(eval_str("42").unwrap(), 42);
    }

    #[test]
    fn addition() {
        assert_eq!(eval_str("1 + 2").unwrap(), 3);
    }

    #[test]
    fn precedence_is_obeyed() {
        assert_eq!(eval_str("1 + 2 * 3").unwrap(), 7);
    }

    #[test]
    fn parentheses_change_the_result() {
        assert_eq!(eval_str("(1 + 2) * 3").unwrap(), 9);
    }

    #[test]
    fn left_associative_subtraction() {
        assert_eq!(eval_str("1 - 2 - 3").unwrap(), -4);
    }

    #[test]
    fn division() {
        assert_eq!(eval_str("10 / 2").unwrap(), 5);
    }

    #[test]
    fn integer_division_truncates() {
        assert_eq!(eval_str("7 / 2").unwrap(), 3);
    }

    #[test]
    fn division_by_zero_errors() {
        assert!(eval_str("10 / 0").is_err());
    }

    #[test]
    fn nested_expression() {
        assert_eq!(eval_str("1 + 2 * (3 - 4)").unwrap(), -1);
    }

    #[test]
    fn simple_negation() {
        assert_eq!(eval_str("-5").unwrap(), -5);
    }

    #[test]
    fn negate_a_group() {
        // -(3 + 2) = -5
        assert_eq!(eval_str("-(3 + 2)").unwrap(), -5);
    }

    #[test]
    fn double_negation_cancels() {
        // --5 = 5
        assert_eq!(eval_str("--5").unwrap(), 5);
    }

    #[test]
    fn negation_precedence() {
        // -2 + 3 = (-2) + 3 = 1, not -(2 + 3) = -5
        assert_eq!(eval_str("-2 + 3").unwrap(), 1);
    }

    #[test]
    fn subtract_a_negative() {
        // 10 - -3 = 13
        assert_eq!(eval_str("10 - -3").unwrap(), 13);
    }

    #[test]
    fn negation_binds_tighter_than_multiply() {
        // -2 * 3 = (-2) * 3 = -6
        assert_eq!(eval_str("-2 * 3").unwrap(), -6);
    }
}