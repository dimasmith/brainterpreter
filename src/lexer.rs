//! Lexer for the l9 source code

use crate::source::Position;
use log::error;

/// Lexical token of the l9 language
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Token {
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
    Number(f64),
    EndOfFile,
    Error,
}

/// Adds debug information to the token
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SourceToken {
    kind: Token,
    source: Position,
}

pub struct Lexer<'a> {
    source: &'a str,
    start: usize,
    pos: usize,
    line: usize,
    column: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Lexer {
            source,
            pos: 0,
            start: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn next_token(&mut self) -> SourceToken {
        let mut maybe_token = self.advance_token();
        while maybe_token.is_none() {
            maybe_token = self.advance_token();
        }
        maybe_token.unwrap()
    }

    fn advance_token(&mut self) -> Option<SourceToken> {
        self.skip_whitespace();
        if self.at_end() {
            return Some(Token::EndOfFile.with_position(self.src_pos()));
        }
        self.start = self.pos;
        let c = self.advance().expect("character exhausted prematurely");
        match c {
            '+' => Some(Token::Plus.with_position(self.src_pos())),
            '-' => Some(Token::Minus.with_position(self.src_pos())),
            '*' => Some(Token::Star.with_position(self.src_pos())),
            '/' => {
                if let Some('/') = self.peek(0) {
                    self.advance();
                    while let Some(c) = self.peek(0) {
                        if c == '\n' {
                            break;
                        }
                        self.advance();
                    }
                    None
                } else {
                    Some(Token::Slash.with_position(self.src_pos()))
                }
            }
            '(' => Some(Token::LParen.with_position(self.src_pos())),
            ')' => Some(Token::RParen.with_position(self.src_pos())),
            '0'..='9' => Some(self.number()),
            _ => {
                error!("unknown token: {}", c);
                Some(Token::Error.with_position(self.src_pos()))
            }
        }
    }

    fn number(&mut self) -> SourceToken {
        while let Some(c) = self.peek(0) {
            if !c.is_ascii_digit() {
                break;
            }
            self.advance();
        }

        if let Some('.') = self.peek(0) {
            self.advance();
            while let Some(c) = self.peek(0) {
                if !c.is_ascii_digit() {
                    break;
                }
                self.advance();
            }
        }
        let number_literal = &self.source[self.start..self.pos];
        let value: f64 = number_literal.parse().expect("must be a correct number");
        Token::Number(value).with_position(self.src_pos())
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.source.chars().nth(self.pos);
        self.pos += 1;
        self.column += 1;
        c
    }

    fn peek(&self, offset: usize) -> Option<char> {
        self.source.chars().nth(self.pos + offset)
    }

    fn at_end(&self) -> bool {
        self.pos >= self.source.len()
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek(0) {
            if !c.is_ascii_whitespace() {
                break;
            }
            if c == '\n' {
                self.line += 1;
                self.column = 0;
            }
            self.advance();
        }
    }

    fn src_pos(&self) -> Position {
        Position::new(self.line, self.column - 1)
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = SourceToken;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next_token() {
            SourceToken {
                kind: Token::EndOfFile,
                ..
            } => None,
            t => Some(t),
        }
    }
}

impl From<Token> for SourceToken {
    fn from(token: Token) -> Self {
        SourceToken {
            kind: token,
            source: Position::default(),
        }
    }
}

impl SourceToken {
    pub fn new(token: Token, source: Position) -> Self {
        SourceToken {
            kind: token,
            source,
        }
    }

    pub fn kind(&self) -> &Token {
        &self.kind
    }

    pub fn source(&self) -> &Position {
        &self.source
    }
}

impl PartialEq<Token> for SourceToken {
    fn eq(&self, other: &Token) -> bool {
        &self.kind == other
    }
}

impl Token {
    pub fn with_position(self, pos: Position) -> SourceToken {
        SourceToken::new(self, pos)
    }

    pub fn with_line(self, line: usize) -> SourceToken {
        SourceToken::new(self, Position::new(line, 0))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn empty_source() {
        let mut lexer = Lexer::new("");
        let token = lexer.next_token();
        assert_eq!(token, Token::EndOfFile);
    }

    #[test]
    fn arithmetic_operators() {
        let mut lexer = Lexer::new("+");
        assert_eq!(lexer.next_token(), Token::Plus);
        assert_eq!(lexer.next_token(), Token::EndOfFile);
    }

    #[test]
    fn integer() {
        let mut lexer = Lexer::new("42");
        assert_eq!(lexer.next_token(), Token::Number(42.0));
        assert_eq!(lexer.next_token(), Token::EndOfFile);
    }

    #[test]
    fn float_point_literal() {
        let mut lexer = Lexer::new("3.14");
        assert_eq!(lexer.next_token(), Token::Number(3.14));
        assert_eq!(lexer.next_token(), Token::EndOfFile);
    }
    #[test]
    fn arithmetic_expressions() {
        let mut lexer = Lexer::new("42 + 8 / 2");
        assert_eq!(lexer.next_token(), Token::Number(42.0));
        assert_eq!(lexer.next_token(), Token::Plus);
        assert_eq!(lexer.next_token(), Token::Number(8.0));
        assert_eq!(lexer.next_token(), Token::Slash);
        assert_eq!(lexer.next_token(), Token::Number(2.0));
    }

    #[test]
    fn inline_comment() {
        let mut lexer = Lexer::new("42 + 7 // this is a comment");
        assert_eq!(lexer.next_token(), Token::Number(42.0));
        assert_eq!(lexer.next_token(), Token::Plus);
        assert_eq!(lexer.next_token(), Token::Number(7.0));
        assert_eq!(lexer.next_token(), Token::EndOfFile);
    }

    #[test]
    fn line_comment() {
        let mut lexer = Lexer::new(
            "// comment
            42 + 7",
        );
        assert_eq!(lexer.next_token(), Token::Number(42.0));
        assert_eq!(lexer.next_token(), Token::Plus);
        assert_eq!(lexer.next_token(), Token::Number(7.0));
        assert_eq!(lexer.next_token(), Token::EndOfFile);
    }
}
