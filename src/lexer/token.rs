// The kinds of tokens the lexer can produce.
// Each variant is one category of "meaningful chunk" in Rynix source.
// Some variants carry data (the actual number or name); most don't.
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Literals
    // The integer is stored as the raw text the user typed, not a parsed
    // i64. This keeps the lexer independent of how integers are ultimately
    // represented, so switching to arbitrary precision later won't touch
    // this phase. The digits become an actual number in a later phase.
    Integer(String),

    // Identifiers: names like `x`, `count`, `main`.
    Identifier(String),

    // Operators
    Plus,     // +
    Minus,    // -
    Star,     // *
    Slash,    // /

    // Grouping
    LeftParen,   // (
    RightParen,  // )

    // Marks the end of input. The parser relies on this to know when to
    // stop, instead of repeatedly checking "are there more tokens?".
    Eof,
}

// A token is a kind plus where it was found in the source.
// The location (line and column) is what lets us later produce
// diagnostics that point at the exact spot, e.g. "line 7, column 14".
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
    pub column: usize,
}

impl Token {
    pub fn new(kind: TokenKind, line: usize, column: usize) -> Token {
        Token { kind, line, column }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_holds_kind_and_location() {
        let tok = Token::new(TokenKind::Plus, 1, 5);
        assert_eq!(tok.kind, TokenKind::Plus);
        assert_eq!(tok.line, 1);
        assert_eq!(tok.column, 5);
    }

    #[test]
    fn integer_token_stores_its_text() {
        let tok = Token::new(TokenKind::Integer("42".to_string()), 3, 1);
        assert_eq!(tok.kind, TokenKind::Integer("42".to_string()));
    }
}