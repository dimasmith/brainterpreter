//! Parser for the l9 interpreter

use crate::ast::AstExpression;
use crate::lexer::Token;
use thiserror::Error;

pub struct Parser<T>
where
    T: Iterator<Item = Token>,
{
    tokens: T,
}

#[derive(Debug, Clone, PartialEq, Error)]
pub enum ParsingError {
    #[error("unknown error during parsing")]
    Unknown,
    #[error("missing operand")]
    MissingOperand,
}

impl<T> Parser<T>
where
    T: Iterator<Item = Token>,
{
    pub fn new(tokens: T) -> Self {
        Parser { tokens }
    }

    pub fn parse(&mut self) -> Result<AstExpression, ParsingError> {
        self.expression()
    }

    fn expression(&mut self) -> Result<AstExpression, ParsingError> {
        if let Some(lhs_token) = self.tokens.next() {
            let lhs = match lhs_token {
                Token::Number(n) => AstExpression::NumberLiteral(n),
                _ => return Err(ParsingError::Unknown),
            };
            if let Some(op_token) = self.tokens.next() {
                let op = match op_token {
                    Token::Plus => {
                        let rhs = self.expression().or(Err(ParsingError::MissingOperand))?;
                        AstExpression::Add(Box::new(lhs), Box::new(rhs))
                    }
                    _ => return Err(ParsingError::Unknown),
                };
                Ok(op)
            } else {
                Ok(lhs)
            }
        } else {
            Err(ParsingError::Unknown)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number_literal() {
        let tokens = vec![Token::Number(42.0)].into_iter();
        let mut parser = Parser::new(tokens);

        let ast = parser.parse().unwrap();

        assert_eq!(ast, AstExpression::NumberLiteral(42.0));
    }

    #[test]
    fn addition_expression() {
        let tokens = vec![Token::Number(7.0), Token::Plus, Token::Number(8.0)].into_iter();
        let mut parser = Parser::new(tokens);

        let ast = parser.parse().unwrap();

        assert_eq!(
            ast,
            AstExpression::Add(
                Box::new(AstExpression::NumberLiteral(7.0)),
                Box::new(AstExpression::NumberLiteral(8.0))
            )
        );
    }

    #[test]
    fn missing_rhs_infix_operation() {
        let tokens = vec![Token::Number(7.0), Token::Plus].into_iter();
        let mut parser = Parser::new(tokens);

        let parsing_error = parser.parse();

        assert_eq!(parsing_error, Err(ParsingError::MissingOperand));
    }
}
