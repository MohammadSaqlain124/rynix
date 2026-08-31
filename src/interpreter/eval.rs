use crate::parser::ast::{BinaryOp, Expr};

// Evaluate an expression tree down to a single integer.
//
// Two rules:
//   - Number  -> interpret its text as an i64.
//   - Binary  -> evaluate both children, then combine with the operator.
//
// The children-first order means precedence (already encoded in the tree
// shape by the parser) is obeyed automatically — this function knows
// nothing about which operator binds tighter.
pub fn eval(expr: &Expr) -> Result<i64, String> {
    match expr {
        // This is where the deferred "integers are i64" decision finally
        // lands. The lexer and parser kept the number as text; the phase
        // that actually does arithmetic is the right place to commit to a
        // representation.
        Expr::Number(text) => text
            .parse::<i64>()
            .map_err(|_| format!("invalid integer literal: {}", text)),

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

    // Helper: lex, parse, and evaluate a source string.
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
        // 1 + 2 * 3 = 7, not 9.
        assert_eq!(eval_str("1 + 2 * 3").unwrap(), 7);
    }

    #[test]
    fn parentheses_change_the_result() {
        // (1 + 2) * 3 = 9.
        assert_eq!(eval_str("(1 + 2) * 3").unwrap(), 9);
    }

    #[test]
    fn left_associative_subtraction() {
        // 1 - 2 - 3 = (1 - 2) - 3 = -4, not 2.
        assert_eq!(eval_str("1 - 2 - 3").unwrap(), -4);
    }

    #[test]
    fn division() {
        assert_eq!(eval_str("10 / 2").unwrap(), 5);
    }

    #[test]
    fn integer_division_truncates() {
        // i64 division discards the remainder: 7 / 2 = 3.
        assert_eq!(eval_str("7 / 2").unwrap(), 3);
    }

    #[test]
    fn division_by_zero_errors() {
        assert!(eval_str("10 / 0").is_err());
    }

    #[test]
    fn nested_expression() {
        // 1 + 2 * (3 - 4) = 1 + 2 * (-1) = 1 + (-2) = -1.
        assert_eq!(eval_str("1 + 2 * (3 - 4)").unwrap(), -1);
    }
}