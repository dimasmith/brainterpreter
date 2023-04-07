use log::trace;

use crate::lexer::token::Token;
use crate::lexer::SourceToken;
use crate::parser::{Parser, ParsingError};

impl<T> Parser<T>
where
    T: Iterator<Item = SourceToken>,
{
    pub fn advance(&mut self) -> SourceToken {
        self.tokens
            .next()
            .unwrap_or(SourceToken::from(Token::EndOfFile))
    }

    pub fn advance_if(&mut self, token: Token) -> bool {
        self.tokens.next_if(|t| t.kind() == &token).is_some()
    }

    pub fn peek(&mut self) -> SourceToken {
        self.tokens
            .peek()
            .cloned()
            .unwrap_or(SourceToken::from(Token::EndOfFile))
    }

    pub fn consume(&mut self, expected: Token) -> Result<(), ParsingError> {
        trace!("Consuming token: {:?}", expected);
        let token = self.advance();
        trace!("Consuming token: current {:?}", token);
        if *token.kind() == expected {
            Ok(())
        } else {
            Err(ParsingError::MissingToken {
                position: *token.source(),
                expected,
                actual: token.kind().clone(),
            })
        }
    }
}
