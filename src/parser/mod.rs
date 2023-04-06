//! Parser for the l9 interpreter

use std::iter::Peekable;

use log::trace;
use thiserror::Error;

use crate::ast::{BinaryOperator, Expression, Program, Statement, UnaryOperator};
use crate::lexer::{SourceToken, Token};
use crate::source::Position;

mod advance;
mod expression;
mod statement;

#[derive(Debug)]
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
        while self.tokens.peek().is_some() {
            program.add_statement(self.statement()?);
        }
        Ok(program)
    }
}
