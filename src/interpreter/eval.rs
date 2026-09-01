use crate::parser::ast::{BinaryOp, Expr, Program, Stmt, UnaryOp};
use std::collections::HashMap;

// The environment: remembers variable values by name.
pub type Env = HashMap<String, i64>;

// Run a whole program, threading one environment through all statements.
// Returns the value of the last statement if it was an expression.
pub fn run(program: &Program) -> Result<Option<i64>, String> {
    let mut env = Env::new();
    let mut last_value = None;

    for stmt in program {
        last_value = exec(stmt, &mut env)?;
    }

    Ok(last_value)
}

// Execute one statement.
fn exec(stmt: &Stmt, env: &mut Env) -> Result<Option<i64>, String> {
    match stmt {
        // `let` DECLARES: always insert (create or overwrite).
        Stmt::Let { name, value } => {
            let v = eval(value, env)?;
            env.insert(name.clone(), v);
            Ok(None)
        }

        // `=` REASSIGNS: only allowed if the variable already exists.
        Stmt::Assign { name, value } => {
            if !env.contains_key(name) {
                return Err(format!("cannot assign to undefined variable: {}", name));
            }
            let v = eval(value, env)?;
            env.insert(name.clone(), v);
            Ok(None)
        }

        Stmt::Expression(expr) => {
            let v = eval(expr, env)?;
            Ok(Some(v))
        }
    }
}

// Evaluate an expression to an integer, using the environment for lookups.
pub fn eval(expr: &Expr, env: &Env) -> Result<i64, String> {
    match expr {
        Expr::Number(text) => text
            .parse::<i64>()
            .map_err(|_| format!("invalid integer literal: {}", text)),

        Expr::Identifier(name) => env
            .get(name)
            .copied()
            .ok_or_else(|| format!("undefined variable: {}", name)),

        Expr::Unary { op, operand } => {
            let value = eval(operand, env)?;
            match op {
                UnaryOp::Negate => Ok(-value),
            }
        }

        Expr::Binary { left, op, right } => {
            let l = eval(left, env)?;
            let r = eval(right, env)?;
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

    fn run_str(source: &str) -> Result<Option<i64>, String> {
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize()?;
        let mut parser = Parser::new(tokens);
        let program = parser.parse()?;
        run(&program)
    }

    fn eval_str(source: &str) -> Result<i64, String> {
        Ok(run_str(source)?.expect("expected a value"))
    }

    #[test]
    fn single_number() {
        assert_eq!(eval_str("42").unwrap(), 42);
    }

    #[test]
    fn precedence_is_obeyed() {
        assert_eq!(eval_str("1 + 2 * 3").unwrap(), 7);
    }

    #[test]
    fn division_by_zero_errors() {
        assert!(eval_str("10 / 0").is_err());
    }

    #[test]
    fn simple_negation() {
        assert_eq!(eval_str("-5").unwrap(), -5);
    }

    #[test]
    fn double_negation_cancels() {
        assert_eq!(eval_str("--5").unwrap(), 5);
    }

    #[test]
    fn negation_precedence() {
        assert_eq!(eval_str("-2 + 3").unwrap(), 1);
    }

    #[test]
    fn variable_depends_on_earlier_variable() {
        assert_eq!(eval_str("let x = 5\nlet y = x + 2\ny * 10").unwrap(), 70);
    }

    #[test]
    fn redefining_with_let_overwrites() {
        assert_eq!(eval_str("let x = 5\nlet x = 10\nx").unwrap(), 10);
    }

    #[test]
    fn undefined_variable_errors() {
        assert!(eval_str("x + 1").is_err());
    }

    // --- New: assignment ---

    #[test]
    fn assignment_changes_value() {
        assert_eq!(eval_str("let x = 5\nx = 10\nx").unwrap(), 10);
    }

    #[test]
    fn assignment_uses_current_values() {
        // x becomes x + 1 using x's current value.
        assert_eq!(eval_str("let x = 5\nx = x + 1\nx").unwrap(), 6);
    }

    #[test]
    fn assignment_affects_later_expression() {
        assert_eq!(eval_str("let x = 5\nx = 20\nx * 2").unwrap(), 40);
    }

    #[test]
    fn assign_to_undefined_variable_errors() {
        // No `let x` first, so `x = 10` must fail.
        assert!(eval_str("x = 10").is_err());
    }

    #[test]
    fn assign_uses_another_variable() {
        assert_eq!(eval_str("let x = 5\nlet y = 3\nx = y + 1\nx").unwrap(), 4);
    }
}