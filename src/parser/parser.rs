use super::ast::{BinaryOp, Expr};
use crate::lexer::token::{Token, TokenKind};

// Grammar we are implementing:
//
//   expression -> term (("+" | "-") term)*
//   term       -> factor (("*" | "/") factor)*
//   factor     -> NUMBER | "(" expression ")"
//
// One function per rule. Deeper rule = tighter binding, so precedence
// falls out of the structure automatically.

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, pos: 0 }
    }

    // Look at the current token without consuming it.
    fn peek(&self) -> &Token {
        &self.tokens[self.pos]
    }

    // Consume the current token and move forward, returning what we
    // consumed.
    fn advance(&mut self) -> Token {
        let token = self.tokens[self.pos].clone();
        // Don't move past the final Eof token.
        if self.pos < self.tokens.len() - 1 {
            self.pos += 1;
        }
        token
    }

    // Entry point: parse a whole expression and make sure nothing is left
    // over (apart from Eof).
    pub fn parse(&mut self) -> Result<Expr, String> {
        let expr = self.parse_expression()?;
        if self.peek().kind != TokenKind::Eof {
            return Err(format!(
                "unexpected token {:?} at line {}, column {}",
                self.peek().kind,
                self.peek().line,
                self.peek().column
            ));
        }
        Ok(expr)
    }

    // expression -> term (("+" | "-") term)*
    fn parse_expression(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_term()?;

        loop {
            let op = match self.peek().kind {
                TokenKind::Plus => BinaryOp::Add,
                TokenKind::Minus => BinaryOp::Subtract,
                _ => break,
            };
            self.advance(); // consume the operator
            let right = self.parse_term()?;
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    // term -> factor (("*" | "/") factor)*
    fn parse_term(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_factor()?;

        loop {
            let op = match self.peek().kind {
                TokenKind::Star => BinaryOp::Multiply,
                TokenKind::Slash => BinaryOp::Divide,
                _ => break,
            };
            self.advance(); // consume the operator
            let right = self.parse_factor()?;
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    // factor -> NUMBER | "(" expression ")"
    fn parse_factor(&mut self) -> Result<Expr, String> {
        let token = self.peek().clone();

        match token.kind {
            TokenKind::Integer(text) => {
                self.advance();
                Ok(Expr::Number(text))
            }
            TokenKind::LeftParen => {
                self.advance(); // consume "("
                let expr = self.parse_expression()?;
                self.expect(TokenKind::RightParen)?;
                Ok(expr)
            }
            _ => Err(format!(
                "expected a number or '(', found {:?} at line {}, column {}",
                token.kind, token.line, token.column
            )),
        }
    }

    // Consume the current token only if it matches the expected kind,
    // otherwise produce an error.
    fn expect(&mut self, kind: TokenKind) -> Result<(), String> {
        if self.peek().kind == kind {
            self.advance();
            Ok(())
        } else {
            Err(format!(
                "expected {:?}, found {:?} at line {}, column {}",
                kind,
                self.peek().kind,
                self.peek().line,
                self.peek().column
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::lexer::Lexer;

    // Helper: lex then parse a source string.
    fn parse_str(source: &str) -> Result<Expr, String> {
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().map_err(|e| e)?;
        let mut parser = Parser::new(tokens);
        parser.parse()
    }

    #[test]
    fn single_number() {
        let expr = parse_str("42").unwrap();
        assert_eq!(expr, Expr::Number("42".to_string()));
    }

    #[test]
    fn simple_addition() {
        // 1 + 2  ->  (+ 1 2)
        let expr = parse_str("1 + 2").unwrap();
        assert_eq!(
            expr,
            Expr::Binary {
                left: Box::new(Expr::Number("1".to_string())),
                op: BinaryOp::Add,
                right: Box::new(Expr::Number("2".to_string())),
            }
        );
    }

    #[test]
    fn precedence_multiply_binds_tighter() {
        // 1 + 2 * 3 must parse as 1 + (2 * 3), NOT (1 + 2) * 3.
        // So the top node is +, and its right child is a *.
        let expr = parse_str("1 + 2 * 3").unwrap();
        match expr {
            Expr::Binary { op: BinaryOp::Add, right, .. } => {
                assert!(matches!(
                    *right,
                    Expr::Binary { op: BinaryOp::Multiply, .. }
                ));
            }
            _ => panic!("expected + at the top with a * on the right"),
        }
    }

    #[test]
    fn parentheses_override_precedence() {
        // (1 + 2) * 3 must parse with * at the top and + as its left child.
        let expr = parse_str("(1 + 2) * 3").unwrap();
        match expr {
            Expr::Binary { op: BinaryOp::Multiply, left, .. } => {
                assert!(matches!(
                    *left,
                    Expr::Binary { op: BinaryOp::Add, .. }
                ));
            }
            _ => panic!("expected * at the top with a + on the left"),
        }
    }

    #[test]
    fn left_associativity() {
        // 1 - 2 - 3 must parse as (1 - 2) - 3, not 1 - (2 - 3).
        // So the top node is a - whose LEFT child is another -.
        let expr = parse_str("1 - 2 - 3").unwrap();
        match expr {
            Expr::Binary { op: BinaryOp::Subtract, left, .. } => {
                assert!(matches!(
                    *left,
                    Expr::Binary { op: BinaryOp::Subtract, .. }
                ));
            }
            _ => panic!("expected left-associative subtraction"),
        }
    }

    #[test]
    fn nested_parentheses() {
        let expr = parse_str("((42))").unwrap();
        assert_eq!(expr, Expr::Number("42".to_string()));
    }

    #[test]
    fn missing_closing_paren_errors() {
        let result = parse_str("(1 + 2");
        assert!(result.is_err());
    }

    #[test]
    fn unexpected_trailing_token_errors() {
        // "1 2" — a second number with no operator between is invalid.
        let result = parse_str("1 2");
        assert!(result.is_err());
    }

    #[test]
    fn empty_input_errors() {
        let result = parse_str("");
        assert!(result.is_err());
    }
}