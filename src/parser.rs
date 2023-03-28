//! Parser for the l9 interpreter

use crate::ast::{Expression, Operation, Program, Statement};
use crate::lexer::{SourceToken, Token};
use crate::source::Position;
use std::iter::Peekable;
use thiserror::Error;

pub struct Parser<T>
where
    T: Iterator<Item = SourceToken>,
{
    tokens: Peekable<T>,
}

#[derive(Debug, Clone, PartialEq, Error)]
pub enum ParsingError {
    #[error("error during parsing at {0}")]
    Unknown(Position),
    #[error("unexpected token `{0:?}` at {1}")]
    UnexpectedToken(Token, Position),
    #[error("missing operand at {0}")]
    MissingOperand(Position),
    #[error("unknown operation at {0}")]
    UnknownOperation(Position),
    #[error("missing closing parentheses at {0}")]
    MissingClosingParentheses(Position),
}

impl<T> Parser<T>
where
    T: Iterator<Item = SourceToken>,
{
    pub fn new(tokens: T) -> Self {
        Parser {
            tokens: tokens.peekable(),
        }
    }

    pub fn parse_program(&mut self) -> Result<Program, ParsingError> {
        let mut program = Program::default();
        while !self.tokens.peek().is_none() {
            program.add_statement(self.statement()?);
        }
        Ok(program)
    }

    pub fn parse(&mut self) -> Result<Statement, ParsingError> {
        self.statement()
    }

    fn statement(&mut self) -> Result<Statement, ParsingError> {
        let token = self.advance();
        let statement = match token.kind() {
            Token::Print => {
                let expr = self.expression(0)?;
                Ok(Statement::Print(expr))
            }
            _ => {
                let expr = self.expression(0)?;
                Ok(Statement::Expression(expr))
            }
        };
        self.consume(Token::Semicolon)?;
        statement
    }

    fn expression(&mut self, min_binding: u8) -> Result<Expression, ParsingError> {
        let token = self.advance();
        let mut lhs = match token.kind() {
            Token::Number(n) => Expression::number(*n),
            Token::Minus => {
                let rhs = self.expression(5)?;
                Expression::unary(Operation::Sub, rhs)
            }
            Token::LParen => {
                let expr = self.expression(0)?;
                match self.advance().kind() {
                    Token::RParen => expr,
                    _ => return Err(ParsingError::MissingClosingParentheses(*token.source())),
                }
            }
            t => {
                return Err(ParsingError::UnexpectedToken(
                    t.clone(),
                    token.source().clone(),
                ))
            }
        };

        loop {
            let token = self.peek();
            let op = match token.kind() {
                Token::Plus => Operation::Add,
                Token::Minus => Operation::Sub,
                Token::Star => Operation::Mul,
                Token::Slash => Operation::Div,
                Token::EndOfFile | Token::RParen | Token::Semicolon => break,
                _ => return Err(ParsingError::UnknownOperation(*token.source())),
            };

            if let Some((left_binding, right_binding)) = self.infix_binding(&op) {
                if left_binding < min_binding {
                    break;
                }

                self.advance();
                let rhs = self
                    .expression(right_binding)
                    .map_err(|_| ParsingError::MissingOperand(*token.source()))?;
                lhs = Expression::binary(op, lhs, rhs);

                continue;
            }

            break;
        }

        Ok(lhs)
    }

    fn infix_binding(&self, op: &Operation) -> Option<(u8, u8)> {
        match op {
            Operation::Add | Operation::Sub => Some((1, 2)),
            Operation::Mul | Operation::Div => Some((3, 4)),
        }
    }

    fn advance(&mut self) -> SourceToken {
        self.tokens
            .next()
            .unwrap_or(SourceToken::from(Token::EndOfFile))
    }

    fn peek(&mut self) -> SourceToken {
        self.tokens
            .peek()
            .cloned()
            .unwrap_or(SourceToken::from(Token::EndOfFile))
    }

    fn consume(&mut self, expected: Token) -> Result<(), ParsingError> {
        let token = self.advance();
        if *token.kind() == expected {
            Ok(())
        } else {
            Err(ParsingError::UnexpectedToken(
                token.kind().clone(),
                token.source().clone(),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Operation;

    #[test]
    fn number_literal() {
        let tokens = vec![Token::Number(42.0).into()].into_iter();
        let mut parser = Parser::new(tokens);

        let ast = parser.expression(0).unwrap();

        assert_eq!(ast, Expression::NumberLiteral(42.0));
    }

    #[test]
    fn addition_expression() {
        let tokens = vec![
            Token::Number(7.0).into(),
            Token::Plus.into(),
            Token::Number(8.0).into(),
        ]
        .into_iter();
        let mut parser = Parser::new(tokens);

        let ast = parser.expression(0).unwrap();

        assert_eq!(
            ast,
            Expression::binary(Operation::Add, Expression::number(7), Expression::number(8))
        );
    }

    #[test]
    fn same_priority_operation_expression() {
        let tokens = vec![
            Token::Number(5.0).into(),
            Token::Plus.into(),
            Token::Number(10.0).into(),
            Token::Minus.into(),
            Token::Number(15.0).into(),
        ]
        .into_iter();
        let mut parser = Parser::new(tokens);

        let ast = parser.expression(0).unwrap();

        assert_eq!(
            ast,
            Expression::binary(
                Operation::Sub,
                Expression::binary(
                    Operation::Add,
                    Expression::number(5),
                    Expression::number(10),
                ),
                Expression::number(15),
            )
        );
    }

    #[test]
    fn missing_rhs_infix_operation() {
        let tokens = vec![
            SourceToken::from(Token::Number(7.0)),
            SourceToken::from(Token::Plus),
        ]
        .into_iter();
        let mut parser = Parser::new(tokens);

        let parsing_error = parser.expression(0);

        assert!(matches!(
            parsing_error,
            Err(ParsingError::MissingOperand(_))
        ));
    }
}
