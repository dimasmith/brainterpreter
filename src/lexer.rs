//! Lexer for the l9 source code

/// Lexical token of the l9 languate
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Plus,
    Number(f64),
    EndOfFile,
    Error,
}

pub struct Lexer<'a> {
    source: &'a str,
    start: usize,
    pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Lexer {
            source,
            pos: 0,
            start: 0,
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        if self.at_end() {
            return Token::EndOfFile;
        }
        self.start = self.pos;
        let c = self.advance().expect("character exhausted prematurely");
        match c {
            '+' => Token::Plus,
            '0'..='9' => self.number(),
            _ => Token::Error,
        }
    }

    fn number(&mut self) -> Token {
        while let Some(c) = self.peek(0) {
            if !c.is_digit(10) {
                break;
            }
            self.advance();
        }
        let number_literal = &self.source[self.start..self.pos];
        let value: f64 = number_literal.parse().expect("must be a correct number");
        Token::Number(value)
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.source.chars().nth(self.pos);
        self.pos += 1;
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
            self.advance();
        }
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
    fn aritmetic_expressions() {
        let mut lexer = Lexer::new("42 + 7");
        assert_eq!(lexer.next_token(), Token::Number(42.0));
        assert_eq!(lexer.next_token(), Token::Plus);
        assert_eq!(lexer.next_token(), Token::Number(7.0));
        assert_eq!(lexer.next_token(), Token::EndOfFile);
    }
}
