use crate::parser::ast::{BinaryOp, Expr, Program, Stmt, UnaryOp};
use std::collections::HashMap;

// The environment: remembers variable values by name. This is the memory
// the evaluator gained when variables arrived — before this, evaluation
// was stateless.
pub type Env = HashMap<String, i64>;

// Run a whole program: execute each statement in order, sharing one
// environment so later statements can see variables from earlier ones.
// Returns the value of the last statement (if it was an expression), so
// the program produces a visible result.
pub fn run(program: &Program) -> Result<Option<i64>, String> {
    let mut env = Env::new();
    let mut last_value = None;

    for stmt in program {
        last_value = exec(stmt, &mut env)?;
    }

    Ok(last_value)
}

// Execute one statement. A `let` defines a variable and produces no value;
// a bare expression is evaluated and its value returned.
fn exec(stmt: &Stmt, env: &mut Env) -> Result<Option<i64>, String> {
    match stmt {
        Stmt::Let { name, value } => {
            let v = eval(value, env)?;
            env.insert(name.clone(), v); // define or overwrite
            Ok(None)
        }
        Stmt::Expression(expr) => {
            let v = eval(expr, env)?;
            Ok(Some(v))
        }
    }
}

// Evaluate an expression to an integer, using the environment to look up
// variables.
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

    // Lex, parse, and run a whole program; return the last value.
    fn run_str(source: &str) -> Result<Option<i64>, String> {
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize()?;
        let mut parser = Parser::new(tokens);
        let program = parser.parse()?;
        run(&program)
    }

    // Convenience for single-expression programs that must yield a value.
    fn eval_str(source: &str) -> Result<i64, String> {
        Ok(run_str(source)?.expect("expected a value"))
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
        assert_eq!(eval_str("-(3 + 2)").unwrap(), -5);
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
    fn subtract_a_negative() {
        assert_eq!(eval_str("10 - -3").unwrap(), 13);
    }

    #[test]
    fn negation_binds_tighter_than_multiply() {
        assert_eq!(eval_str("-2 * 3").unwrap(), -6);
    }

    // --- New: variables ---

    #[test]
    fn let_then_use() {
        assert_eq!(eval_str("let x = 5\nx").unwrap(), 5);
    }

    #[test]
    fn variable_in_expression() {
        assert_eq!(eval_str("let x = 5\nx + 2").unwrap(), 7);
    }

    #[test]
    fn variable_depends_on_earlier_variable() {
        assert_eq!(eval_str("let x = 5\nlet y = x + 2\ny * 10").unwrap(), 70);
    }

    #[test]
    fn redefining_overwrites() {
        // let-defines-or-overwrites: the second let changes x.
        assert_eq!(eval_str("let x = 5\nlet x = 10\nx").unwrap(), 10);
    }

    #[test]
    fn undefined_variable_errors() {
        assert!(eval_str("x + 1").is_err());
    }

    #[test]
    fn let_produces_no_value_but_program_runs() {
        // A program ending in a `let` yields no value (None), not an error.
        assert_eq!(run_str("let x = 5").unwrap(), None);
    }
}