//! Parser for the l9 interpreter

use crate::ast::{AstExpression, Operation};
use crate::lexer::Token;
use std::iter::Peekable;
use thiserror::Error;

pub struct Parser<T>
where
    T: Iterator<Item = Token>,
{
    tokens: Peekable<T>,
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
        Parser {
            tokens: tokens.peekable(),
        }
    }

    pub fn parse(&mut self) -> Result<AstExpression, ParsingError> {
        self.expression(0)
    }

    fn expression(&mut self, min_binding: u8) -> Result<AstExpression, ParsingError> {
        let mut lhs = match self.advance() {
            Token::Number(n) => AstExpression::NumberLiteral(n),
            _ => return Err(ParsingError::Unknown),
        };

        loop {
            let op = match self.peek() {
                Token::Plus => Operation::Add,
                Token::Minus => Operation::Sub,
                Token::EndOfFile => break,
                _ => return Err(ParsingError::Unknown),
            };

            let (left_binding, right_binding) = self.infix_binding(&op);

            if left_binding < min_binding {
                break;
            }

            self.advance();
            let rhs = self
                .expression(right_binding)
                .map_err(|_| ParsingError::MissingOperand)?;
            lhs = AstExpression::BinaryOperation(op, Box::new(lhs), Box::new(rhs));
        }

        Ok(lhs)
    }

    fn infix_binding(&self, op: &Operation) -> (u8, u8) {
        match op {
            Operation::Add | Operation::Sub => (1, 2),
        }
    }

    fn advance(&mut self) -> Token {
        self.tokens.next().unwrap_or(Token::EndOfFile)
    }

    fn peek(&mut self) -> Token {
        self.tokens.peek().cloned().unwrap_or(Token::EndOfFile)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Operation;

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
            AstExpression::binary_op(
                Operation::Add,
                AstExpression::number(7),
                AstExpression::number(8)
            )
        );
    }

    #[test]
    fn same_priority_operation_expression() {
        let tokens = vec![
            Token::Number(5.0),
            Token::Plus,
            Token::Number(10.0),
            Token::Minus,
            Token::Number(15.0),
        ]
        .into_iter();
        let mut parser = Parser::new(tokens);

        let ast = parser.parse().unwrap();

        assert_eq!(
            ast,
            AstExpression::binary_op(
                Operation::Sub,
                AstExpression::binary_op(
                    Operation::Add,
                    AstExpression::number(5),
                    AstExpression::number(10),
                ),
                AstExpression::number(15),
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
