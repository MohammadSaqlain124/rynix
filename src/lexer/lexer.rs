use super::token::{Token, TokenKind};

pub struct Lexer {
    // The source, as a list of characters so we can index it simply.
    chars: Vec<char>,
    // How far we've read.
    pos: usize,
    // Current location in the source, for diagnostics.
    line: usize,
    column: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Lexer {
        Lexer {
            chars: source.chars().collect(),
            pos: 0,
            line: 1,
            column: 1,
        }
    }

    // Look at the current character without consuming it.
    // Returns None if we've reached the end of input.
    fn peek(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

    // Consume the current character and advance the cursor,
    // keeping line/column up to date.
    fn advance(&mut self) -> Option<char> {
        let c = self.peek()?;
        self.pos += 1;
        if c == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        Some(c)
    }

    // Turn the whole source into a list of tokens.
    // Returns an error message if it hits a character it doesn't recognize.
    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();

        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                self.advance();
            } else if c.is_ascii_digit() {
                tokens.push(self.read_number());
            } else if let Some(kind) = single_char_token(c) {
                let line = self.line;
                let column = self.column;
                self.advance();
                tokens.push(Token::new(kind, line, column));
            } else {
                return Err(format!(
                    "unexpected character '{}' at line {}, column {}",
                    c, self.line, self.column
                ));
            }
        }

        tokens.push(Token::new(TokenKind::Eof, self.line, self.column));
        Ok(tokens)
    }

    // Read a run of digits into one Integer token.
    // We record the start position so the token points at the number's
    // first digit.
    fn read_number(&mut self) -> Token {
        let line = self.line;
        let column = self.column;
        let mut text = String::new();

        while let Some(c) = self.peek() {
            if c.is_ascii_digit() {
                text.push(c);
                self.advance();
            } else {
                break;
            }
        }

        Token::new(TokenKind::Integer(text), line, column)
    }
}

// Map a single character to its token kind, if it is one.
fn single_char_token(c: char) -> Option<TokenKind> {
    match c {
        '+' => Some(TokenKind::Plus),
        '-' => Some(TokenKind::Minus),
        '*' => Some(TokenKind::Star),
        '/' => Some(TokenKind::Slash),
        '(' => Some(TokenKind::LeftParen),
        ')' => Some(TokenKind::RightParen),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input_gives_only_eof() {
        let mut lexer = Lexer::new("");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens, vec![Token::new(TokenKind::Eof, 1, 1)]);
    }

    #[test]
    fn single_number() {
        let mut lexer = Lexer::new("42");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].kind, TokenKind::Integer("42".to_string()));
    }

    #[test]
    fn multi_digit_number_is_one_token() {
        let mut lexer = Lexer::new("12345");
        let tokens = lexer.tokenize().unwrap();
        // One Integer token plus the Eof token.
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].kind, TokenKind::Integer("12345".to_string()));
    }

    #[test]
    fn operators_and_numbers() {
        let mut lexer = Lexer::new("1 + 2 * 3");
        let tokens = lexer.tokenize().unwrap();
        let kinds: Vec<TokenKind> = tokens.into_iter().map(|t| t.kind).collect();
        assert_eq!(
            kinds,
            vec![
                TokenKind::Integer("1".to_string()),
                TokenKind::Plus,
                TokenKind::Integer("2".to_string()),
                TokenKind::Star,
                TokenKind::Integer("3".to_string()),
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn parentheses() {
        let mut lexer = Lexer::new("(1)");
        let tokens = lexer.tokenize().unwrap();
        let kinds: Vec<TokenKind> = tokens.into_iter().map(|t| t.kind).collect();
        assert_eq!(
            kinds,
            vec![
                TokenKind::LeftParen,
                TokenKind::Integer("1".to_string()),
                TokenKind::RightParen,
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn unexpected_character_errors() {
        let mut lexer = Lexer::new("1 @ 2");
        let result = lexer.tokenize();
        assert!(result.is_err());
    }
}