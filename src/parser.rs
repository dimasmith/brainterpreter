//! Parser for the l9 interpreter

use std::iter::Peekable;

use log::trace;
use thiserror::Error;

use crate::ast::{BinaryOperator, Expression, Program, Statement, UnaryOperator};
use crate::lexer::{SourceToken, Token};
use crate::source::Position;

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

    pub fn parse(&mut self) -> Result<Statement, ParsingError> {
        self.statement()
    }

    fn statement(&mut self) -> Result<Statement, ParsingError> {
        let token = self.advance();
        match token.kind() {
            Token::Print => {
                trace!("Parsing print statement");
                let expr = self.expression(0)?;
                self.consume(Token::Semicolon)?;
                Ok(Statement::Print(expr))
            }
            Token::LeftCurly => self.block_statement(),
            Token::Let => {
                let declaration = self.variable_declaration();
                self.consume(Token::Semicolon)?;
                declaration
            }
            Token::Fun => self.function_declaration(),
            Token::If => self.if_statement(),
            Token::While => self.while_statement(),
            Token::Identifier(name) => {
                if let Some(Token::LeftParen) = self.tokens.peek().map(|t| t.kind()) {
                    self.function_call(name)
                } else {
                    self.variable_assignment(&token, name)
                }
            }
            Token::Return => {
                let expr = self.expression(0)?;
                self.consume(Token::Semicolon)?;
                Ok(Statement::Return(expr))
            }
            _ => Err(ParsingError::Unknown(*token.source())),
        }
    }

    fn variable_assignment(
        &mut self,
        token: &SourceToken,
        name: &str,
    ) -> Result<Statement, ParsingError> {
        match self.advance().kind() {
            Token::Equal => {
                let expr = self.expression(0)?;
                let assignment = Statement::Assignment(name.to_string(), expr);
                self.consume(Token::Semicolon)?;
                Ok(assignment)
            }
            _ => Err(ParsingError::Unknown(*token.source())),
        }
    }

    fn function_call(&mut self, name: &str) -> Result<Statement, ParsingError> {
        let mut arguments = vec![];
        self.consume(Token::LeftParen)?;
        if let Some(Token::RightParen) = self.tokens.peek().map(|t| t.kind()) {
            self.consume(Token::RightParen)?;
            self.consume(Token::Semicolon)?;
            return Ok(Statement::FunctionCall(name.to_string(), arguments));
        }
        loop {
            let expr = self.expression(0)?;
            arguments.push(expr);
            let token = self.advance();
            match token.kind() {
                Token::Comma => continue,
                Token::RightParen => break,
                _ => return Err(ParsingError::MissingClosingParentheses(*token.source())),
            }
        }
        self.consume(Token::RightParen)?;
        self.consume(Token::Semicolon)?;
        Ok(Statement::FunctionCall(name.to_string(), arguments))
    }

    fn function_call_expression(&mut self, name: &str) -> Result<Expression, ParsingError> {
        trace!("Parsing function call expression (name: {})", name);
        let mut arguments = vec![];
        self.consume(Token::LeftParen)?;
        if let Some(Token::RightParen) = self.tokens.peek().map(|t| t.kind()) {
            self.consume(Token::RightParen)?;
            self.consume(Token::Semicolon)?;
            return Ok(Expression::FunctionCall(name.to_string(), arguments));
        }
        loop {
            let expr = self.expression(0)?;
            arguments.push(expr);
            let token = self.advance();
            match token.kind() {
                Token::Comma => continue,
                Token::RightParen => break,
                _ => return Err(ParsingError::MissingClosingParentheses(*token.source())),
            }
        }
        Ok(Expression::FunctionCall(name.to_string(), arguments))
    }

    fn expression(&mut self, min_binding: u8) -> Result<Expression, ParsingError> {
        trace!("Parsing expression (min_binding: {})", min_binding);
        let token = self.advance();
        trace!("Parsing expression (token: {:?})", token);
        let mut lhs = match token.kind() {
            Token::Number(n) => Expression::number(*n),
            Token::Nil => Expression::Nil,
            Token::True => Expression::BooleanLiteral(true),
            Token::False => Expression::BooleanLiteral(false),
            Token::StringLiteral(s) => Expression::StringLiteral(s.clone()),
            Token::Minus => {
                let binding = self.prefix_binding(token.clone())?;
                let rhs = self.expression(binding)?;
                Expression::unary(UnaryOperator::Negate, rhs)
            }
            Token::Bang => {
                let binding = self.prefix_binding(token.clone())?;
                let rhs = self.expression(binding)?;
                Expression::unary(UnaryOperator::Not, rhs)
            }
            Token::Identifier(name) => {
                if let Some(Token::LeftParen) = self.tokens.peek().map(|t| t.kind()) {
                    self.function_call_expression(name)?
                } else {
                    Expression::Variable(name.clone())
                }
            }
            Token::LeftParen => {
                let expr = self.expression(0)?;
                match self.advance().kind() {
                    Token::RightParen => expr,
                    _ => return Err(ParsingError::MissingClosingParentheses(*token.source())),
                }
            }
            t => return Err(ParsingError::UnexpectedToken(t.clone(), *token.source())),
        };

        loop {
            let mut token = self.peek();

            if let Some(left_binding) = self.postfix_binding(&token) {
                if left_binding < min_binding {
                    break;
                }
                if let Token::LeftSquare = token.kind() {
                    self.advance();
                    let index = self.expression(0)?;
                    self.consume(Token::RightSquare)?;
                    lhs = Expression::ArrayIndex(Box::new(lhs), Box::new(index));
                    token = self.peek();
                }
            }

            if let Some((left_binding, right_binding)) = self.infix_binding(&token) {
                if left_binding < min_binding {
                    break;
                }
                let op = self
                    .binary_operator()
                    .ok_or_else(|| ParsingError::Unknown(*token.source()))?;
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

    fn binary_operator(&mut self) -> Option<BinaryOperator> {
        let token = self.peek();
        match token.kind() {
            Token::Plus => Some(BinaryOperator::Add),
            Token::Minus => Some(BinaryOperator::Sub),
            Token::Star => Some(BinaryOperator::Mul),
            Token::Slash => Some(BinaryOperator::Div),
            Token::EqualEqual => Some(BinaryOperator::Equal),
            Token::BangEqual => Some(BinaryOperator::NotEqual),
            Token::Less => Some(BinaryOperator::Less),
            Token::LessEqual => Some(BinaryOperator::LessOrEqual),
            Token::Greater => Some(BinaryOperator::Greater),
            Token::GreaterEqual => Some(BinaryOperator::GreaterOrEqual),
            _ => None,
        }
    }

    fn block_statement(&mut self) -> Result<Statement, ParsingError> {
        trace!("Parsing block statement");
        let mut statements = Vec::new();
        loop {
            match self.peek().kind() {
                Token::RightCurly | Token::EndOfFile => {
                    break;
                }
                _ => {}
            }
            statements.push(self.statement()?);
        }
        self.consume(Token::RightCurly)?;
        Ok(Statement::Block(statements))
    }

    fn infix_binding(&self, token: &SourceToken) -> Option<(u8, u8)> {
        match token.kind() {
            Token::Plus | Token::Minus => Some((3, 4)),
            Token::Star | Token::Slash => Some((5, 6)),
            Token::EqualEqual | Token::BangEqual => Some((1, 2)),
            Token::Less | Token::LessEqual => Some((1, 2)),
            Token::Greater | Token::GreaterEqual => Some((1, 2)),
            _ => None,
        }
    }

    fn postfix_binding(&self, token: &SourceToken) -> Option<u8> {
        match token.kind() {
            Token::LeftSquare => Some(7),
            _ => None,
        }
    }

    fn prefix_binding(&self, token: SourceToken) -> Result<u8, ParsingError> {
        match token.kind() {
            Token::Minus | Token::Bang => Ok(7),
            _ => Err(ParsingError::UnknownOperation(*token.source())),
        }
    }

    fn variable_declaration(&mut self) -> Result<Statement, ParsingError> {
        trace!("Parsing variable declaration");
        let token = self.advance();
        trace!("Variable declaration token: {:?}", token);
        let name = match token.kind() {
            Token::Identifier(name) => name,
            _ => {
                return Err(ParsingError::UnexpectedToken(
                    token.kind().clone(),
                    *token.source(),
                ))
            }
        };

        trace!("Assigning variable: {:?}", token);
        if self.advance_if(Token::Equal) {
            let expr = self.expression(0)?;
            Ok(Statement::Declaration(name.clone(), Some(expr)))
        } else {
            Ok(Statement::Declaration(name.clone(), None))
        }
    }

    fn function_declaration(&mut self) -> Result<Statement, ParsingError> {
        trace!("Parsing function declaration");
        let token = self.advance();
        trace!("Function declaration token: {:?}", token);
        let name = match token.kind() {
            Token::Identifier(name) => name,
            _ => {
                return Err(ParsingError::UnexpectedToken(
                    token.kind().clone(),
                    *token.source(),
                ))
            }
        };

        let mut parameters = vec![];
        self.consume(Token::LeftParen)?;
        if let Token::Identifier(name) = self.peek().kind() {
            parameters.push(name.clone());
            self.advance();
        }
        while self.advance_if(Token::Comma) {
            if let Token::Identifier(name) = self.peek().kind() {
                parameters.push(name.clone());
                self.advance();
            }
        }
        self.consume(Token::RightParen)?;
        self.consume(Token::LeftCurly)?;
        let body = self.block_statement()?;
        Ok(Statement::FunctionDeclaration(
            name.clone(),
            parameters,
            vec![body],
        ))
    }

    fn if_statement(&mut self) -> Result<Statement, ParsingError> {
        trace!("Parsing if statement");
        self.consume(Token::LeftParen)?;
        let condition = self.expression(0)?;
        self.consume(Token::RightParen)?;
        let then_branch = self.statement()?;
        let else_branch = if self.advance_if(Token::Else) {
            Some(Box::new(self.statement()?))
        } else {
            None
        };
        Ok(Statement::If(condition, Box::new(then_branch), else_branch))
    }

    fn while_statement(&mut self) -> Result<Statement, ParsingError> {
        trace!("Parsing while statement");
        self.consume(Token::LeftParen)?;
        let condition = self.expression(0)?;
        self.consume(Token::RightParen)?;
        let body = self.statement()?;
        Ok(Statement::While(condition, Box::new(body)))
    }

    fn advance(&mut self) -> SourceToken {
        self.tokens
            .next()
            .unwrap_or(SourceToken::from(Token::EndOfFile))
    }

    fn advance_if(&mut self, token: Token) -> bool {
        self.tokens.next_if(|t| t.kind() == &token).is_some()
    }

    fn peek(&mut self) -> SourceToken {
        self.tokens
            .peek()
            .cloned()
            .unwrap_or(SourceToken::from(Token::EndOfFile))
    }

    fn consume(&mut self, expected: Token) -> Result<(), ParsingError> {
        trace!("Consuming token: {:?}", expected);
        let token = self.advance();
        trace!("Consuming token: current {:?}", token);
        if *token.kind() == expected {
            Ok(())
        } else {
            Err(ParsingError::UnexpectedToken(
                token.kind().clone(),
                *token.source(),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::BinaryOperator;

    use super::*;

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
            Expression::binary(
                BinaryOperator::Add,
                Expression::number(7),
                Expression::number(8)
            )
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
                BinaryOperator::Sub,
                Expression::binary(
                    BinaryOperator::Add,
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

    #[test]
    fn print_variable() {
        let tokens = vec![
            Token::Print.into(),
            Token::Identifier("x".to_string()).into(),
            Token::Semicolon.into(),
        ]
        .into_iter();
        let mut parser = Parser::new(tokens);

        let ast = parser.statement().unwrap();

        assert_eq!(ast, Statement::Print(Expression::Variable("x".to_string())));
    }

    #[test]
    fn function_declaration_no_parameters() {
        let tokens = vec![
            Token::Fun.into(),
            Token::Identifier("foo".to_string()).into(),
            Token::LeftParen.into(),
            Token::RightParen.into(),
            Token::LeftCurly.into(),
            Token::Print.into(),
            Token::Identifier("x".to_string()).into(),
            Token::Semicolon.into(),
            Token::RightCurly.into(),
        ]
        .into_iter();
        let mut parser = Parser::new(tokens);

        let ast = parser.statement().unwrap();

        assert_eq!(
            ast,
            Statement::FunctionDeclaration(
                "foo".to_string(),
                vec![],
                vec![Statement::Block(vec![Statement::Print(
                    Expression::Variable("x".to_string())
                )])]
            )
        );
    }

    #[test]
    fn function_declaration_single_parameter() {
        let tokens = vec![
            Token::Fun.into(),
            Token::Identifier("foo".to_string()).into(),
            Token::LeftParen.into(),
            Token::Identifier("a".to_string()).into(),
            Token::RightParen.into(),
            Token::LeftCurly.into(),
            Token::Print.into(),
            Token::Identifier("x".to_string()).into(),
            Token::Semicolon.into(),
            Token::RightCurly.into(),
        ]
        .into_iter();
        let mut parser = Parser::new(tokens);

        let ast = parser.statement().unwrap();

        assert_eq!(
            ast,
            Statement::FunctionDeclaration(
                "foo".to_string(),
                vec!["a".to_string()],
                vec![Statement::Block(vec![Statement::Print(
                    Expression::Variable("x".to_string())
                )])]
            )
        );
    }

    #[test]
    fn function_declaration_multiple_parameters() {
        let tokens = vec![
            Token::Fun.into(),
            Token::Identifier("foo".to_string()).into(),
            Token::LeftParen.into(),
            Token::Identifier("a".to_string()).into(),
            Token::Comma.into(),
            Token::Identifier("b".to_string()).into(),
            Token::RightParen.into(),
            Token::LeftCurly.into(),
            Token::Print.into(),
            Token::Identifier("x".to_string()).into(),
            Token::Semicolon.into(),
            Token::RightCurly.into(),
        ]
        .into_iter();
        let mut parser = Parser::new(tokens);

        let ast = parser.statement().unwrap();

        assert_eq!(
            ast,
            Statement::FunctionDeclaration(
                "foo".to_string(),
                vec!["a".to_string(), "b".to_string()],
                vec![Statement::Block(vec![Statement::Print(
                    Expression::Variable("x".to_string())
                )])]
            )
        );
    }
}
