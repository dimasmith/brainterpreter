use log::trace;

use crate::ast::{BinaryOperator, Expression, UnaryOperator};
use crate::lexer::token::Token;
use crate::lexer::SourceToken;
use crate::parser::{Parser, ParsingError};

type ParsingResult = Result<Expression, ParsingError>;

impl<T> Parser<T>
where
    T: Iterator<Item = SourceToken>,
{
    pub fn expression(&mut self) -> ParsingResult {
        self.expression_bp(0)
    }

    fn expression_bp(&mut self, min_binding: u8) -> ParsingResult {
        trace!("Parsing expression (min_binding: {min_binding})");
        let token = self.advance();
        trace!("Parsing expression (token: {token:?})");
        let mut lhs = match token {
            Token::Number(n) => Expression::number(n),
            Token::Nil => Expression::Nil,
            Token::True => Expression::BooleanLiteral(true),
            Token::False => Expression::BooleanLiteral(false),
            Token::StringLiteral(s) => Expression::StringLiteral(s),
            Token::Minus | Token::Bang => self.unary_operation(&token)?,
            Token::Identifier(name) => Expression::Variable(name),
            Token::LeftParen => self.grouping()?,
            Token::LeftSquare => self.array_initialisation()?,
            t => return Err(ParsingError::UnexpectedToken(t, self.last_position())),
        };

        loop {
            if let Some(left_binding) = self.postfix_binding() {
                if left_binding < min_binding {
                    break;
                }
                if self.advance_if(Token::LeftSquare) {
                    lhs = self.index(lhs)?;
                    continue;
                }
                if self.advance_if(Token::LeftParen) {
                    lhs = self.call(lhs)?;
                    continue;
                }
            }

            if let Some((left_binding, right_binding)) = self.infix_binding() {
                if left_binding < min_binding {
                    break;
                }
                if self.advance_if(Token::Equal) {
                    lhs = self.assignment(lhs, right_binding)?;
                    continue;
                }
                lhs = self.binary_operation(lhs, right_binding)?;
                continue;
            }

            break;
        }

        Ok(lhs)
    }

    fn grouping(&mut self) -> ParsingResult {
        let expr = self.expression_bp(0)?;
        self.consume(&Token::RightParen)?;
        Ok(expr)
    }

    fn unary_operation(&mut self, token: &Token) -> ParsingResult {
        let binding = self
            .prefix_binding(token)
            .ok_or(ParsingError::UnknownOperation(self.last_position()))?;
        let rhs = self.expression_bp(binding)?;
        let operator = match token {
            Token::Minus => UnaryOperator::Negate,
            Token::Bang => UnaryOperator::Not,
            _ => return Err(ParsingError::UnknownOperation(self.last_position())),
        };
        Ok(Expression::unary(operator, rhs))
    }

    fn binary_operation(&mut self, lhs: Expression, right_binding: u8) -> ParsingResult {
        let op = self
            .binary_operator()
            .ok_or_else(|| ParsingError::Unknown(self.last_position()))?;
        self.advance();
        let rhs = self
            .expression_bp(right_binding)
            .map_err(|_| ParsingError::MissingOperand(self.last_position()))?;

        Ok(Expression::binary(op, lhs, rhs))
    }

    fn assignment(&mut self, lhs: Expression, right_binding: u8) -> ParsingResult {
        if let Expression::Variable(name) = lhs {
            let rhs = self.expression_bp(right_binding)?;
            return Ok(Expression::AssignVariable(name, Box::new(rhs)));
        }

        if let Expression::Index { array, index } = lhs {
            if let Expression::Variable(name) = *array {
                let rhs = self.expression_bp(right_binding)?;
                return Ok(Expression::AssignIndexVariable {
                    variable: name,
                    index,
                    value: Box::new(rhs),
                });
            }
        }

        Err(ParsingError::InvalidAssignment(self.last_position()))
    }

    fn index(&mut self, lhs: Expression) -> ParsingResult {
        let index = self.expression_bp(0)?;
        self.consume(&Token::RightSquare)?;
        Ok(Expression::Index {
            array: Box::new(lhs),
            index: Box::new(index),
        })
    }

    fn array_initialisation(&mut self) -> ParsingResult {
        let initial = self.expression_bp(0)?;
        self.consume(&Token::Semicolon)?;
        let size = self.expression_bp(0)?;
        self.consume(&Token::RightSquare)?;
        Ok(Expression::Array {
            initial: Box::new(initial),
            size: Box::new(size),
        })
    }

    fn call(&mut self, lhs: Expression) -> ParsingResult {
        match lhs {
            Expression::Variable(name) => self.function_call(&name),
            _ => Err(ParsingError::InvalidCall(self.last_position())),
        }
    }

    fn function_call(&mut self, name: &str) -> ParsingResult {
        trace!("Parsing function call expression (name: {name})");
        let mut arguments = vec![];
        if let Token::RightParen = self.peek() {
            self.consume(&Token::RightParen)?;
            return Ok(Expression::FunctionCall(name.to_string(), arguments));
        }
        loop {
            let expr = self.expression_bp(0)?;
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
        Ok(Expression::FunctionCall(name.to_string(), arguments))
    }

    fn binary_operator(&mut self) -> Option<BinaryOperator> {
        match self.peek() {
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

    fn infix_binding(&mut self) -> Option<(u8, u8)> {
        match self.peek() {
            Token::Plus | Token::Minus => Precedence::Term.infix_binding(),
            Token::Star | Token::Slash => Precedence::Factor.infix_binding(),
            Token::EqualEqual | Token::BangEqual => Precedence::Equality.infix_binding(),
            Token::Less | Token::LessEqual => Precedence::Comparison.infix_binding(),
            Token::Greater | Token::GreaterEqual => Precedence::Comparison.infix_binding(),
            Token::Equal => Precedence::Assignment.infix_binding(),
            _ => None,
        }
    }

    fn postfix_binding(&mut self) -> Option<u8> {
        match self.peek() {
            Token::LeftSquare => Precedence::Index.postfix_binding(),
            Token::LeftParen => Precedence::Call.postfix_binding(),
            _ => None,
        }
    }

    fn prefix_binding(&self, token: &Token) -> Option<u8> {
        match token {
            Token::Minus | Token::Bang => Precedence::Unary.prefix_binding(),
            _ => None,
        }
    }
}

enum Precedence {
    Assignment,
    Equality,
    Comparison,
    Term,
    Factor,
    Unary,
    Call,
    Index,
}

impl Precedence {
    fn base_binding(&self) -> u8 {
        match self {
            // Precedence::None => 0,
            Precedence::Assignment => 1,
            // Precedence::Or => 3,
            // Precedence::And => 5,
            Precedence::Equality => 7,
            Precedence::Comparison => 9,
            Precedence::Term => 11,
            Precedence::Factor => 13,
            Precedence::Unary => 15,
            Precedence::Call => 17,
            Precedence::Index => 19,
        }
    }

    fn infix_binding(&self) -> Option<(u8, u8)> {
        match self {
            Precedence::Unary | Precedence::Index => None,
            p => Some((p.base_binding(), p.base_binding() + 1)),
        }
    }

    fn prefix_binding(&self) -> Option<u8> {
        match self {
            Precedence::Unary => Some(self.base_binding()),
            _ => None,
        }
    }

    fn postfix_binding(&self) -> Option<u8> {
        match self {
            Precedence::Index | Precedence::Call => Some(self.base_binding()),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::Expression;
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    use super::*;

    #[test]
    fn unary_operation() {
        let mut parser = Parser::new(Lexer::new("-1"));
        let expr = parser.expression().unwrap();
        assert_eq!(
            expr,
            Expression::unary(UnaryOperator::Negate, Expression::number(1))
        );
    }

    #[test]
    fn binary_operation() {
        let mut parser = Parser::new(Lexer::new("1 + 2"));
        let expr = parser.expression().unwrap();
        assert_eq!(
            expr,
            Expression::binary(
                BinaryOperator::Add,
                Expression::number(1),
                Expression::number(2)
            )
        );
    }

    #[test]
    fn operation_priorities() {
        let mut parser = Parser::new(Lexer::new("1 + -2 * 3"));
        let expr = parser.expression().unwrap();
        assert_eq!(
            expr,
            Expression::binary(
                BinaryOperator::Add,
                Expression::number(1),
                Expression::binary(
                    BinaryOperator::Mul,
                    Expression::unary(UnaryOperator::Negate, Expression::number(2)),
                    Expression::number(3)
                )
            )
        );
    }

    #[test]
    fn grouping() {
        let mut parser = Parser::new(Lexer::new("(1 + 2) * 3"));
        let expr = parser.expression().unwrap();
        assert_eq!(
            expr,
            Expression::binary(
                BinaryOperator::Mul,
                Expression::binary(
                    BinaryOperator::Add,
                    Expression::number(1),
                    Expression::number(2)
                ),
                Expression::number(3)
            )
        );
    }

    #[test]
    fn variable_assignment() {
        let mut parser = Parser::new(Lexer::new("a = 1"));
        let expr = parser.expression().unwrap();
        assert_eq!(
            expr,
            Expression::AssignVariable("a".to_string(), Box::new(Expression::number(1)))
        );
    }

    #[test]
    fn indexed_access() {
        let mut parser = Parser::new(Lexer::new("a[1]"));
        let expr = parser.expression().unwrap();
        assert_eq!(
            expr,
            Expression::Index {
                array: Box::new(Expression::variable("a")),
                index: Box::new(Expression::number(1))
            }
        );
    }

    #[test]
    fn indexed_assignment() {
        let mut parser = Parser::new(Lexer::new("a[1] = 2"));
        let expr = parser.expression().unwrap();
        assert_eq!(
            expr,
            Expression::AssignIndexVariable {
                variable: "a".to_string(),
                index: Box::new(Expression::number(1)),
                value: Box::new(Expression::number(2))
            }
        );
    }

    #[test]
    fn function_call() {
        let mut parser = Parser::new(Lexer::new("foo()"));
        let expr = parser.expression().unwrap();
        assert_eq!(expr, Expression::FunctionCall("foo".to_string(), vec![]));
    }

    #[test]
    fn function_call_with_arguments() {
        let mut parser = Parser::new(Lexer::new("foo(1, 2)"));
        let expr = parser.expression().unwrap();
        assert_eq!(
            expr,
            Expression::FunctionCall(
                "foo".to_string(),
                vec![Expression::number(1), Expression::number(2)]
            )
        );
    }

    #[test]
    fn array_initialisation() {
        let mut parser = Parser::new(Lexer::new("[1; 5]"));
        let expr = parser.expression().unwrap();
        assert_eq!(
            expr,
            Expression::Array {
                size: Box::new(Expression::number(5)),
                initial: Box::new(Expression::number(1))
            }
        );
    }
}
