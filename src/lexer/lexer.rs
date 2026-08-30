use super::token::{Token, TokenKind};

pub struct Lexer {
    chars: Vec<char>,
    pos: usize,
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

    fn peek(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

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

    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();

        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                self.advance();
            } else if c == '~' {
                self.skip_comment()?;
            } else if c.is_ascii_digit() {
                tokens.push(self.read_number());
            } else if is_identifier_start(c) {
                tokens.push(self.read_identifier_or_keyword());
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

    // Skip a comment delimited by ~ on both sides, e.g. ~ like this ~.
    // The comment may span multiple lines. If the closing ~ is never
    // found, that's an error rather than silently eating the rest of
    // the file.
    fn skip_comment(&mut self) -> Result<(), String> {
        let start_line = self.line;
        let start_column = self.column;
        self.advance(); // consume the opening ~

        while let Some(c) = self.peek() {
            if c == '~' {
                self.advance(); // consume the closing ~
                return Ok(());
            }
            self.advance();
        }

        Err(format!(
            "unterminated comment starting at line {}, column {}",
            start_line, start_column
        ))
    }

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

    fn read_identifier_or_keyword(&mut self) -> Token {
        let line = self.line;
        let column = self.column;
        let mut text = String::new();

        while let Some(c) = self.peek() {
            if is_identifier_continue(c) {
                text.push(c);
                self.advance();
            } else {
                break;
            }
        }

        let kind = keyword_kind(&text).unwrap_or(TokenKind::Identifier(text));
        Token::new(kind, line, column)
    }
}

fn single_char_token(c: char) -> Option<TokenKind> {
    match c {
        '+' => Some(TokenKind::Plus),
        '-' => Some(TokenKind::Minus),
        '*' => Some(TokenKind::Star),
        '/' => Some(TokenKind::Slash),
        '=' => Some(TokenKind::Equals),
        '(' => Some(TokenKind::LeftParen),
        ')' => Some(TokenKind::RightParen),
        _ => None,
    }
}

fn is_identifier_start(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}

fn is_identifier_continue(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}

fn keyword_kind(word: &str) -> Option<TokenKind> {
    match word {
        "let" => Some(TokenKind::Let),
        "const" => Some(TokenKind::Const),
        "if" => Some(TokenKind::If),
        "else" => Some(TokenKind::Else),
        "while" => Some(TokenKind::While),
        "for" => Some(TokenKind::For),
        "fn" => Some(TokenKind::Fn),
        "return" => Some(TokenKind::Return),
        "true" => Some(TokenKind::True),
        "false" => Some(TokenKind::False),
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

    #[test]
    fn identifier_is_recognized() {
        let mut lexer = Lexer::new("count");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].kind, TokenKind::Identifier("count".to_string()));
    }

    #[test]
    fn identifier_with_digits_and_underscore() {
        let mut lexer = Lexer::new("_temp1");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].kind, TokenKind::Identifier("_temp1".to_string()));
    }

    #[test]
    fn keywords_are_distinguished_from_identifiers() {
        let mut lexer = Lexer::new("let x");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].kind, TokenKind::Let);
        assert_eq!(tokens[1].kind, TokenKind::Identifier("x".to_string()));
    }

    #[test]
    fn lettuce_is_not_the_let_keyword() {
        let mut lexer = Lexer::new("lettuce");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].kind, TokenKind::Identifier("lettuce".to_string()));
    }

    #[test]
    fn number_next_to_identifier_splits_correctly() {
        let mut lexer = Lexer::new("1x");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].kind, TokenKind::Integer("1".to_string()));
        assert_eq!(tokens[1].kind, TokenKind::Identifier("x".to_string()));
    }

    #[test]
    fn comment_is_skipped() {
        let mut lexer = Lexer::new("1 ~ this is ignored ~ 2");
        let tokens = lexer.tokenize().unwrap();
        let kinds: Vec<TokenKind> = tokens.into_iter().map(|t| t.kind).collect();
        assert_eq!(
            kinds,
            vec![
                TokenKind::Integer("1".to_string()),
                TokenKind::Integer("2".to_string()),
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn multiline_comment_is_skipped() {
        let mut lexer = Lexer::new("1 ~ spanning\ntwo lines ~ 2");
        let tokens = lexer.tokenize().unwrap();
        let kinds: Vec<TokenKind> = tokens.into_iter().map(|t| t.kind).collect();
        assert_eq!(
            kinds,
            vec![
                TokenKind::Integer("1".to_string()),
                TokenKind::Integer("2".to_string()),
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn unterminated_comment_errors() {
        let mut lexer = Lexer::new("1 ~ oops no closing");
        let result = lexer.tokenize();
        assert!(result.is_err());
    }

    #[test]
    fn assignment_expression() {
        let mut lexer = Lexer::new("let count = 42");
        let tokens = lexer.tokenize().unwrap();
        let kinds: Vec<TokenKind> = tokens.into_iter().map(|t| t.kind).collect();
        assert_eq!(
            kinds,
            vec![
                TokenKind::Let,
                TokenKind::Identifier("count".to_string()),
                TokenKind::Equals,
                TokenKind::Integer("42".to_string()),
                TokenKind::Eof,
            ]
        );
    }
}