use super::ast::{BinaryOp, Expr, Program, Stmt, UnaryOp};
use crate::lexer::token::{Token, TokenKind};

// Grammar we are implementing:
//
//   program              -> statement*
//   statement            -> let_statement | expression_statement
//   let_statement        -> "let" IDENTIFIER "=" expression
//   expression_statement -> expression
//
//   expression -> term (("+" | "-") term)*
//   term       -> unary (("*" | "/") unary)*
//   unary      -> "-" unary | factor
//   factor     -> NUMBER | IDENTIFIER | "(" expression ")"

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

    // program -> statement*
    // Parse statements until we reach Eof.
    pub fn parse(&mut self) -> Result<Program, String> {
        let mut statements = Vec::new();
        while self.peek().kind != TokenKind::Eof {
            statements.push(self.parse_statement()?);
        }
        Ok(statements)
    }

    // statement -> let_statement | expression_statement
    fn parse_statement(&mut self) -> Result<Stmt, String> {
        if self.peek().kind == TokenKind::Let {
            self.parse_let_statement()
        } else {
            let expr = self.parse_expression()?;
            Ok(Stmt::Expression(expr))
        }
    }

    // let_statement -> "let" IDENTIFIER "=" expression
    fn parse_let_statement(&mut self) -> Result<Stmt, String> {
        self.advance(); // consume "let"

        // Expect an identifier for the variable name.
        let name = match self.peek().kind.clone() {
            TokenKind::Identifier(name) => {
                self.advance();
                name
            }
            _ => {
                return Err(format!(
                    "expected a variable name after 'let', found {:?} at line {}, column {}",
                    self.peek().kind,
                    self.peek().line,
                    self.peek().column
                ));
            }
        };

        self.expect(TokenKind::Equals)?;
        let value = self.parse_expression()?;
        Ok(Stmt::Let { name, value })
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
            self.advance();
            let operand = self.parse_unary()?;
            Ok(Expr::Unary {
                op: UnaryOp::Negate,
                operand: Box::new(operand),
            })
        } else {
            self.parse_factor()
        }
    }

    // factor -> NUMBER | IDENTIFIER | "(" expression ")"
    fn parse_factor(&mut self) -> Result<Expr, String> {
        let token = self.peek().clone();

        match token.kind {
            TokenKind::Integer(text) => {
                self.advance();
                Ok(Expr::Number(text))
            }
            TokenKind::Identifier(name) => {
                self.advance();
                Ok(Expr::Identifier(name))
            }
            TokenKind::LeftParen => {
                self.advance();
                let expr = self.parse_expression()?;
                self.expect(TokenKind::RightParen)?;
                Ok(expr)
            }
            _ => Err(format!(
                "expected a number, name, or '(', found {:?} at line {}, column {}",
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

    fn parse_str(source: &str) -> Result<Program, String> {
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize()?;
        let mut parser = Parser::new(tokens);
        parser.parse()
    }

    // Helper: parse a single-statement program and return that statement.
    fn parse_one(source: &str) -> Stmt {
        let program = parse_str(source).unwrap();
        assert_eq!(program.len(), 1, "expected exactly one statement");
        program.into_iter().next().unwrap()
    }

    #[test]
    fn single_number_is_expression_statement() {
        let stmt = parse_one("42");
        assert_eq!(stmt, Stmt::Expression(Expr::Number("42".to_string())));
    }

    #[test]
    fn simple_addition() {
        let stmt = parse_one("1 + 2");
        assert_eq!(
            stmt,
            Stmt::Expression(Expr::Binary {
                left: Box::new(Expr::Number("1".to_string())),
                op: BinaryOp::Add,
                right: Box::new(Expr::Number("2".to_string())),
            })
        );
    }

    #[test]
    fn precedence_multiply_binds_tighter() {
        let stmt = parse_one("1 + 2 * 3");
        match stmt {
            Stmt::Expression(Expr::Binary { op: BinaryOp::Add, right, .. }) => {
                assert!(matches!(*right, Expr::Binary { op: BinaryOp::Multiply, .. }));
            }
            _ => panic!("expected + at the top with a * on the right"),
        }
    }

    #[test]
    fn parentheses_override_precedence() {
        let stmt = parse_one("(1 + 2) * 3");
        match stmt {
            Stmt::Expression(Expr::Binary { op: BinaryOp::Multiply, left, .. }) => {
                assert!(matches!(*left, Expr::Binary { op: BinaryOp::Add, .. }));
            }
            _ => panic!("expected * at the top with a + on the left"),
        }
    }

    #[test]
    fn left_associativity() {
        let stmt = parse_one("1 - 2 - 3");
        match stmt {
            Stmt::Expression(Expr::Binary { op: BinaryOp::Subtract, left, .. }) => {
                assert!(matches!(*left, Expr::Binary { op: BinaryOp::Subtract, .. }));
            }
            _ => panic!("expected left-associative subtraction"),
        }
    }

    #[test]
    fn simple_negation() {
        let stmt = parse_one("-5");
        assert_eq!(
            stmt,
            Stmt::Expression(Expr::Unary {
                op: UnaryOp::Negate,
                operand: Box::new(Expr::Number("5".to_string())),
            })
        );
    }

    #[test]
    fn double_negation() {
        let stmt = parse_one("--5");
        match stmt {
            Stmt::Expression(Expr::Unary { op: UnaryOp::Negate, operand }) => {
                assert!(matches!(*operand, Expr::Unary { op: UnaryOp::Negate, .. }));
            }
            _ => panic!("expected nested negation"),
        }
    }

    #[test]
    fn negation_binds_tighter_than_plus() {
        let stmt = parse_one("-2 + 3");
        match stmt {
            Stmt::Expression(Expr::Binary { op: BinaryOp::Add, left, .. }) => {
                assert!(matches!(*left, Expr::Unary { op: UnaryOp::Negate, .. }));
            }
            _ => panic!("expected + at the top with a negation on the left"),
        }
    }

    #[test]
    fn identifier_is_an_expression() {
        let stmt = parse_one("x");
        assert_eq!(stmt, Stmt::Expression(Expr::Identifier("x".to_string())));
    }

    #[test]
    fn let_statement() {
        let stmt = parse_one("let x = 5");
        assert_eq!(
            stmt,
            Stmt::Let {
                name: "x".to_string(),
                value: Expr::Number("5".to_string()),
            }
        );
    }

    #[test]
    fn let_with_expression_value() {
        // let y = x + 2
        let stmt = parse_one("let y = x + 2");
        match stmt {
            Stmt::Let { name, value } => {
                assert_eq!(name, "y");
                assert!(matches!(value, Expr::Binary { op: BinaryOp::Add, .. }));
            }
            _ => panic!("expected a let statement"),
        }
    }

    #[test]
    fn multiple_statements() {
        let program = parse_str("let x = 5\nlet y = x + 2\ny * 10").unwrap();
        assert_eq!(program.len(), 3);
        assert!(matches!(program[0], Stmt::Let { .. }));
        assert!(matches!(program[1], Stmt::Let { .. }));
        assert!(matches!(program[2], Stmt::Expression(_)));
    }

    #[test]
    fn let_without_name_errors() {
        assert!(parse_str("let = 5").is_err());
    }

    #[test]
    fn let_without_equals_errors() {
        assert!(parse_str("let x 5").is_err());
    }

    #[test]
    fn missing_closing_paren_errors() {
        assert!(parse_str("(1 + 2").is_err());
    }
}