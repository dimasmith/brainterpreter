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
            Token::Print => self.print_statement(),
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
        Ok(Statement::Function(name, parameters, Box::new(body)))
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

    fn print_statement(&mut self) -> Result<Statement, ParsingError> {
        trace!("Parsing print statement");
        let expr = self.expression()?;
        self.consume(&Token::Semicolon)?;
        Ok(Statement::Print(expr))
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
    use crate::ast::{BinaryOperator, Expression, Statement};
    use crate::lexer::Lexer;

    use super::*;

    #[test]
    fn variable_declaration() {
        let mut parser = Parser::new(Lexer::new("let a;"));
        let statement = parser.statement().unwrap();
        assert_eq!(statement, Statement::Variable("a".to_string(), None));
    }

    #[test]
    fn variable_assignment() {
        let mut parser = Parser::new(Lexer::new("a = 1;"));
        let statement = parser.statement().unwrap();
        assert_eq!(
            statement,
            Statement::Expression(Expression::Assign {
                target: Box::new(Expression::variable("a")),
                value: Box::new(Expression::number(1))
            })
        );
    }

    #[test]
    fn variable_definition() {
        let mut parser = Parser::new(Lexer::new("let a = 1;"));
        let statement = parser.statement().unwrap();
        assert_eq!(
            statement,
            Statement::Variable("a".to_string(), Some(Expression::number(1)))
        );
    }

    #[test]
    fn function_definition() {
        let mut parser = Parser::new(Lexer::new("fun a() {}"));
        let statement = parser.statement().unwrap();
        assert_eq!(
            statement,
            Statement::function("a", &[], Statement::Block(vec![]))
        );
    }

    #[test]
    fn function_definition_with_parameters() {
        let mut parser = Parser::new(Lexer::new("fun a(b, c) {}"));
        let statement = parser.statement().unwrap();
        assert_eq!(
            statement,
            Statement::function("a", &["b", "c"], Statement::Block(vec![]))
        );
    }

    #[test]
    fn if_statement() {
        let mut parser = Parser::new(Lexer::new("if (a == 10) { }"));
        let statement = parser.statement().unwrap();
        assert_eq!(
            statement,
            Statement::if_statement(
                Expression::binary(
                    BinaryOperator::Equal,
                    Expression::variable("a"),
                    Expression::number(10)
                ),
                Statement::Block(vec![]),
            )
        );
    }

    #[test]
    fn if_else_statement() {
        let mut parser = Parser::new(Lexer::new("if (a == 10) { } else {}"));
        let statement = parser.statement().unwrap();
        assert_eq!(
            statement,
            Statement::if_else_statement(
                Expression::binary(
                    BinaryOperator::Equal,
                    Expression::variable("a"),
                    Expression::number(10)
                ),
                Statement::Block(vec![]),
                Statement::Block(vec![])
            )
        );
    }

    #[test]
    fn while_statement() {
        let mut parser = Parser::new(Lexer::new("while (i > 0) { }"));
        let statement = parser.statement().unwrap();
        assert_eq!(
            statement,
            Statement::while_loop(
                Expression::binary(
                    BinaryOperator::Greater,
                    Expression::variable("i"),
                    Expression::number(0)
                ),
                Statement::Block(vec![])
            )
        );
    }

    #[test]
    fn print_statement() {
        let mut parser = Parser::new(Lexer::new("print 1;"));
        let statement = parser.statement().unwrap();
        assert_eq!(statement, Statement::Print(Expression::number(1)));
    }
}
