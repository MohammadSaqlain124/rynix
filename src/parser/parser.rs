use super::ast::{BinaryOp, Expr, UnaryOp};
use crate::lexer::token::{Token, TokenKind};

// Grammar we are implementing:
//
//   expression -> term (("+" | "-") term)*
//   term       -> unary (("*" | "/") unary)*
//   unary      -> "-" unary | factor
//   factor     -> NUMBER | "(" expression ")"
//
// One function per rule. Deeper rule = tighter binding, so precedence
// falls out of the structure automatically. `unary` sits below `term`,
// so negation binds tighter than * and /.

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, pos: 0 }
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.pos]
    }

    fn advance(&mut self) -> Token {
        let token = self.tokens[self.pos].clone();
        if self.pos < self.tokens.len() - 1 {
            self.pos += 1;
        }
        token
    }

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
            self.advance();
            let right = self.parse_term()?;
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    // term -> unary (("*" | "/") unary)*
    fn parse_term(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_unary()?;

        loop {
            let op = match self.peek().kind {
                TokenKind::Star => BinaryOp::Multiply,
                TokenKind::Slash => BinaryOp::Divide,
                _ => break,
            };
            self.advance();
            let right = self.parse_unary()?;
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    // unary -> "-" unary | factor
    fn parse_unary(&mut self) -> Result<Expr, String> {
        if self.peek().kind == TokenKind::Minus {
            self.advance(); // consume the "-"
            let operand = self.parse_unary()?; // recurse: handles --5, etc.
            Ok(Expr::Unary {
                op: UnaryOp::Negate,
                operand: Box::new(operand),
            })
        } else {
            self.parse_factor()
        }
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
                self.advance();
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

    fn parse_str(source: &str) -> Result<Expr, String> {
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize()?;
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
        let expr = parse_str("1 + 2 * 3").unwrap();
        match expr {
            Expr::Binary { op: BinaryOp::Add, right, .. } => {
                assert!(matches!(*right, Expr::Binary { op: BinaryOp::Multiply, .. }));
            }
            _ => panic!("expected + at the top with a * on the right"),
        }
    }

    #[test]
    fn parentheses_override_precedence() {
        let expr = parse_str("(1 + 2) * 3").unwrap();
        match expr {
            Expr::Binary { op: BinaryOp::Multiply, left, .. } => {
                assert!(matches!(*left, Expr::Binary { op: BinaryOp::Add, .. }));
            }
            _ => panic!("expected * at the top with a + on the left"),
        }
    }

    #[test]
    fn left_associativity() {
        let expr = parse_str("1 - 2 - 3").unwrap();
        match expr {
            Expr::Binary { op: BinaryOp::Subtract, left, .. } => {
                assert!(matches!(*left, Expr::Binary { op: BinaryOp::Subtract, .. }));
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
    fn simple_negation() {
        // -5  ->  Unary(Negate, 5)
        let expr = parse_str("-5").unwrap();
        assert_eq!(
            expr,
            Expr::Unary {
                op: UnaryOp::Negate,
                operand: Box::new(Expr::Number("5".to_string())),
            }
        );
    }

    #[test]
    fn double_negation() {
        // --5  ->  Unary(Negate, Unary(Negate, 5))
        let expr = parse_str("--5").unwrap();
        match expr {
            Expr::Unary { op: UnaryOp::Negate, operand } => {
                assert!(matches!(*operand, Expr::Unary { op: UnaryOp::Negate, .. }));
            }
            _ => panic!("expected nested negation"),
        }
    }

    #[test]
    fn negation_binds_tighter_than_plus() {
        // -2 + 3 must parse as (-2) + 3, so the top is + with a Unary left.
        let expr = parse_str("-2 + 3").unwrap();
        match expr {
            Expr::Binary { op: BinaryOp::Add, left, .. } => {
                assert!(matches!(*left, Expr::Unary { op: UnaryOp::Negate, .. }));
            }
            _ => panic!("expected + at the top with a negation on the left"),
        }
    }

    #[test]
    fn subtraction_of_a_negative() {
        // 10 - -3 is valid: binary minus, then unary minus.
        let expr = parse_str("10 - -3").unwrap();
        match expr {
            Expr::Binary { op: BinaryOp::Subtract, right, .. } => {
                assert!(matches!(*right, Expr::Unary { op: UnaryOp::Negate, .. }));
            }
            _ => panic!("expected subtraction with a negated right operand"),
        }
    }

    #[test]
    fn missing_closing_paren_errors() {
        assert!(parse_str("(1 + 2").is_err());
    }

    #[test]
    fn unexpected_trailing_token_errors() {
        assert!(parse_str("1 2").is_err());
    }

    #[test]
    fn empty_input_errors() {
        assert!(parse_str("").is_err());
    }
}