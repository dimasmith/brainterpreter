use log::trace;

use crate::ast::Statement;
use crate::lexer::token::Token;
use crate::lexer::SourceToken;
use crate::parser::{Parser, ParsingError};

impl<T> Parser<T>
where
    T: Iterator<Item = SourceToken>,
{
    pub fn statement(&mut self) -> Result<Statement, ParsingError> {
        match self.advance() {
            Token::Print => {
                trace!("Parsing print statement");
                let expr = self.expression()?;
                self.consume(&Token::Semicolon)?;
                Ok(Statement::Print(expr))
            }
            Token::LeftCurly => self.block_statement(),
            Token::Let => {
                let declaration = self.variable_declaration();
                self.consume(&Token::Semicolon)?;
                declaration
            }
            Token::Fun => self.function_declaration(),
            Token::If => self.if_statement(),
            Token::While => self.while_statement(),
            Token::Identifier(name) => {
                if let Some(Token::LeftParen) = self.tokens.peek().map(|t| t.kind()) {
                    self.function_call(&name)
                } else {
                    // &self.advance()
                    self.variable_assignment(&name)
                }
            }
            Token::Return => {
                let expr = self.expression()?;
                self.consume(&Token::Semicolon)?;
                Ok(Statement::Return(expr))
            }
            _ => Err(ParsingError::Unknown(self.last_position())),
        }
    }

    fn variable_assignment(&mut self, name: &str) -> Result<Statement, ParsingError> {
        match self.advance() {
            Token::Equal => {
                let expr = self.expression()?;
                let assignment = Statement::Assignment(name.to_string(), expr);
                self.consume(&Token::Semicolon)?;
                Ok(assignment)
            }
            t => Err(ParsingError::MissingToken {
                position: self.last_position(),
                expected: Token::Equal,
                actual: t.clone(),
            }),
        }
    }

    fn function_call(&mut self, name: &str) -> Result<Statement, ParsingError> {
        let mut arguments = vec![];
        self.consume(&Token::LeftParen)?;
        if let Token::RightParen = self.peek() {
            self.consume(&Token::RightParen)?;
            self.consume(&Token::Semicolon)?;
            return Ok(Statement::FunctionCall(name.to_string(), arguments));
        }
        loop {
            let expr = self.expression()?;
            arguments.push(expr);
            let token = self.advance();
            match token {
                Token::Comma => continue,
                Token::RightParen => break,
                _ => {
                    return Err(ParsingError::MissingClosingParentheses(
                        self.last_position(),
                    ))
                }
            }
        }
        self.consume(&Token::RightParen)?;
        self.consume(&Token::Semicolon)?;
        Ok(Statement::FunctionCall(name.to_string(), arguments))
    }

    fn block_statement(&mut self) -> Result<Statement, ParsingError> {
        trace!("Parsing block statement");
        let mut statements = Vec::new();
        loop {
            match self.peek() {
                Token::RightCurly | Token::EndOfFile => {
                    break;
                }
                _ => {}
            }
            statements.push(self.statement()?);
        }
        self.consume(&Token::RightCurly)?;
        Ok(Statement::Block(statements))
    }

    fn variable_declaration(&mut self) -> Result<Statement, ParsingError> {
        trace!("Parsing variable declaration");
        let token = self.advance();
        trace!("Variable declaration token: {:?}", token);
        let name = match token {
            Token::Identifier(name) => name.clone(),
            _ => {
                return Err(ParsingError::UnexpectedToken(
                    token.clone(),
                    self.last_position(),
                ))
            }
        };

        if self.advance_if(Token::Equal) {
            let expr = self.expression()?;
            Ok(Statement::Declaration(name, Some(expr)))
        } else {
            Ok(Statement::Declaration(name, None))
        }
    }

    fn function_declaration(&mut self) -> Result<Statement, ParsingError> {
        trace!("Parsing function declaration");
        let token = self.advance();
        trace!("Function declaration token: {:?}", token);
        let name = match token {
            Token::Identifier(name) => name,
            _ => {
                return Err(ParsingError::UnexpectedToken(
                    token.clone(),
                    self.last_position(),
                ))
            }
        };

        let mut parameters = vec![];
        self.consume(&Token::LeftParen)?;
        if let Token::Identifier(name) = self.peek() {
            parameters.push(name.clone());
            self.advance();
        }
        while self.advance_if(Token::Comma) {
            if let Token::Identifier(name) = self.peek() {
                parameters.push(name.clone());
                self.advance();
            }
        }
        self.consume(&Token::RightParen)?;
        self.consume(&Token::LeftCurly)?;
        let body = self.block_statement()?;
        Ok(Statement::FunctionDeclaration(
            name.clone(),
            parameters,
            vec![body],
        ))
    }

    fn if_statement(&mut self) -> Result<Statement, ParsingError> {
        trace!("Parsing if statement");
        self.consume(&Token::LeftParen)?;
        let condition = self.expression()?;
        self.consume(&Token::RightParen)?;
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
        self.consume(&Token::LeftParen)?;
        let condition = self.expression()?;
        self.consume(&Token::RightParen)?;
        let body = self.statement()?;
        Ok(Statement::While(condition, Box::new(body)))
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::Expression;
    use crate::lexer::token::Token;
    use crate::lexer::{Lexer, SourceToken};

    use super::*;

    #[test]
    fn test_variable_declaration() {
        let mut parser = Parser::new(Lexer::new("let a;"));
        let statement = parser.statement().unwrap();
        assert_eq!(statement, Statement::Declaration("a".to_string(), None));
    }

    #[test]
    fn test_variable_assignment() {
        let mut parser = Parser::new(Lexer::new("a = 1;"));
        let statement = parser.statement().unwrap();
        assert_eq!(
            statement,
            Statement::Assignment("a".to_string(), Expression::NumberLiteral(1.0))
        );
    }

    #[test]
    fn test_variable_declaration_with_assignment() {
        let mut parser = Parser::new(Lexer::new("let a = 1;"));
        let statement = parser.statement().unwrap();
        assert_eq!(
            statement,
            Statement::Declaration("a".to_string(), Some(Expression::NumberLiteral(1.0)))
        );
    }

    #[test]
    fn test_function_declaration() {
        let mut parser = Parser::new(Lexer::new("fun a() {}"));
        let statement = parser.statement().unwrap();
        assert_eq!(
            statement,
            Statement::FunctionDeclaration("a".to_string(), vec![], vec![Statement::Block(vec![])])
        );
    }

    #[test]
    fn test_function_declaration_with_parameters() {
        let mut parser = Parser::new(Lexer::new("fun a(b, c) {}"));
        let statement = parser.statement().unwrap();
        assert_eq!(
            statement,
            Statement::FunctionDeclaration(
                "a".to_string(),
                vec!["b".to_string(), "c".to_string()],
                vec![Statement::Block(vec![])]
            )
        );
    }
}
