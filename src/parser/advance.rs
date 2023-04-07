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
        trace!("Consuming token: {:?}", expected);
        let token = self.advance();
        trace!("Consuming token: current {:?}", token);
        if &token == expected {
            return Ok(());
        }
        Err(ParsingError::MissingToken {
            position: self.last_position(),
            expected: expected.clone(),
            actual: token.clone(),
        })
    }

    pub fn last_position(&mut self) -> Position {
        self.tokens
            .peek()
            .map(|t| *t.source())
            .unwrap_or(Position::default())
    }
}
