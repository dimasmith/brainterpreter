use log::trace;

use crate::lexer::token::Token;
use crate::lexer::SourceToken;
use crate::parser::{Parser, ParsingError};
use crate::source::Position;

impl<T> Parser<T>
where
    T: Iterator<Item = SourceToken>,
{
    pub fn advance(&mut self) -> Token {
        self.tokens
            .next()
            .map(|t| t.kind().clone())
            .unwrap_or(Token::EndOfFile)
    }

    pub fn advance_if(&mut self, token: Token) -> bool {
        self.tokens.next_if(|t| t.kind() == &token).is_some()
    }

    pub fn peek(&mut self) -> &Token {
        self.tokens
            .peek()
            .map(|t| t.kind())
            .unwrap_or(&Token::EndOfFile)
    }

    pub fn consume(&mut self, expected: &Token) -> Result<(), ParsingError> {
        trace!("Consuming token: {expected:?}");
        let token = self.advance();
        trace!("Consuming token: current {token:?}");
        if &token == expected {
            return Ok(());
        }
        Err(ParsingError::MissingToken {
            position: self.last_position(),
            expected: expected.clone(),
            actual: token,
        })
    }

    pub fn last_position(&mut self) -> Position {
        self.tokens.peek().map(|t| *t.source()).unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    #[test]
    fn parser_advance() {
        let lexer = Lexer::new("1 + 2");
        let mut parser = Parser::new(lexer);
        assert_eq!(parser.advance(), Token::Number(1.0));
        assert_eq!(parser.advance(), Token::Plus);
        assert_eq!(parser.advance(), Token::Number(2.0));
        assert_eq!(parser.advance(), Token::EndOfFile);
    }

    #[test]
    fn consume_token() {
        let tokens = vec![
            SourceToken::new(Token::Number(1.0), Position::default()),
            SourceToken::new(Token::Plus, Position::default()),
            SourceToken::new(Token::Number(2.0), Position::default()),
        ];
        let mut parser = Parser::new(tokens.into_iter());
        parser.advance();
        parser.consume(&Token::Plus).unwrap();
        assert_eq!(parser.peek(), &Token::Number(2.0));
    }

    #[test]
    fn consume_wrong_token() {
        let tokens = vec![
            SourceToken::new(Token::Number(1.0), Position::default()),
            SourceToken::new(Token::Plus, Position::default()),
            SourceToken::new(Token::Number(2.0), Position::default()),
        ];
        let mut parser = Parser::new(tokens.into_iter());
        parser.advance();
        let result = parser.consume(&Token::Minus);

        assert_eq!(
            result,
            Err(ParsingError::MissingToken {
                position: Position::default(),
                expected: Token::Minus,
                actual: Token::Plus,
            })
        );
    }

    #[test]
    fn advance_if_match() {
        let tokens = vec![
            SourceToken::new(Token::Plus, Position::default()),
            SourceToken::new(Token::Number(2.0), Position::default()),
        ];
        let mut parser = Parser::new(tokens.into_iter());
        parser.advance_if(Token::Plus);
        assert_eq!(parser.peek(), &Token::Number(2.0));
    }

    #[test]
    fn advance_if_no_match() {
        let tokens = vec![
            SourceToken::new(Token::Plus, Position::default()),
            SourceToken::new(Token::Number(2.0), Position::default()),
        ];
        let mut parser = Parser::new(tokens.into_iter());
        parser.advance_if(Token::Minus);
        assert_eq!(parser.peek(), &Token::Plus);
    }
}
