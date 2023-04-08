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
        if let Token::Identifier(_) = self.peek() {
            return self.expression_statement();
        }
        match self.advance() {
            Token::Print => {
                trace!("Parsing print statement");
                let expr = self.expression()?;
                self.consume(&Token::Semicolon)?;
                Ok(Statement::Print(expr))
            }
            Token::LeftCurly => self.block_statement(),
            Token::Let => self.variable_definition(),
            Token::Fun => self.function_definition(),
            Token::If => self.if_statement(),
            Token::While => self.while_statement(),
            Token::Return => {
                let expr = self.expression()?;
                self.consume(&Token::Semicolon)?;
                Ok(Statement::Return(expr))
            }
            _ => Err(ParsingError::Unknown(self.last_position())),
        }
    }

    fn variable_definition(&mut self) -> Result<Statement, ParsingError> {
        trace!("Parsing variable declaration");
        let token = self.advance();
        trace!("Variable declaration token: {:?}", token);
        let name = match token {
            Token::Identifier(name) => name,
            _ => {
                return Err(ParsingError::MissingToken {
                    position: self.last_position(),
                    expected: Token::Identifier("identifier".to_string()),
                    actual: token.clone(),
                })
            }
        };

        let def = if self.advance_if(Token::Equal) {
            let expr = self.expression()?;
            Ok(Statement::Variable(name, Some(expr)))
        } else {
            Ok(Statement::Variable(name, None))
        };
        self.consume(&Token::Semicolon)?;
        def
    }

    fn function_definition(&mut self) -> Result<Statement, ParsingError> {
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
        Ok(Statement::Function(name, parameters, vec![body]))
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

    fn expression_statement(&mut self) -> Result<Statement, ParsingError> {
        trace!("Parsing expression statement");
        let expr = self.expression()?;
        self.consume(&Token::Semicolon)?;
        Ok(Statement::Expression(expr))
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::{Expression, Statement};
    use crate::lexer::Lexer;

    use super::*;

    #[test]
    fn test_variable_declaration() {
        let mut parser = Parser::new(Lexer::new("let a;"));
        let statement = parser.statement().unwrap();
        assert_eq!(statement, Statement::Variable("a".to_string(), None));
    }

    #[test]
    fn test_variable_assignment() {
        let mut parser = Parser::new(Lexer::new("a = 1;"));
        let statement = parser.statement().unwrap();
        assert_eq!(
            statement,
            Statement::Expression(Expression::Assign {
                target: Box::new(Expression::Variable("a".to_string())),
                value: Box::new(Expression::NumberLiteral(1.0))
            })
        );
    }

    #[test]
    fn test_variable_declaration_with_assignment() {
        let mut parser = Parser::new(Lexer::new("let a = 1;"));
        let statement = parser.statement().unwrap();
        assert_eq!(
            statement,
            Statement::Variable("a".to_string(), Some(Expression::NumberLiteral(1.0)))
        );
    }

    #[test]
    fn test_function_declaration() {
        let mut parser = Parser::new(Lexer::new("fun a() {}"));
        let statement = parser.statement().unwrap();
        assert_eq!(
            statement,
            Statement::Function("a".to_string(), vec![], vec![Statement::Block(vec![])])
        );
    }

    #[test]
    fn test_function_declaration_with_parameters() {
        let mut parser = Parser::new(Lexer::new("fun a(b, c) {}"));
        let statement = parser.statement().unwrap();
        assert_eq!(
            statement,
            Statement::Function(
                "a".to_string(),
                vec!["b".to_string(), "c".to_string()],
                vec![Statement::Block(vec![])]
            )
        );
    }
}
