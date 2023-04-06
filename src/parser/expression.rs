use log::trace;

use crate::ast::{BinaryOperator, Expression, UnaryOperator};
use crate::lexer::{SourceToken, Token};
use crate::parser::{Parser, ParsingError};

impl<T> Parser<T>
where
    T: Iterator<Item = SourceToken>,
{
    pub fn expression(&mut self) -> Result<Expression, ParsingError> {
        self.expression_bp(0)
    }

    fn expression_bp(&mut self, min_binding: u8) -> Result<Expression, ParsingError> {
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
                let binding = self
                    .prefix_binding(token.clone())
                    .ok_or(ParsingError::UnknownOperation(*token.source()))?;
                let rhs = self.expression_bp(binding)?;
                Expression::unary(UnaryOperator::Negate, rhs)
            }
            Token::Bang => {
                let binding = self
                    .prefix_binding(token.clone())
                    .ok_or(ParsingError::UnknownOperation(*token.source()))?;
                let rhs = self.expression_bp(binding)?;
                Expression::unary(UnaryOperator::Not, rhs)
            }
            Token::Identifier(name) => self.variable_expression(name)?,
            Token::LeftParen => {
                let expr = self.expression_bp(0)?;
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
                    let index = self.expression_bp(0)?;
                    self.consume(Token::RightSquare)?;
                    lhs = Expression::Index(Box::new(lhs), Box::new(index));
                    token = self.peek();
                } else if let Token::LeftParen = token.kind() {
                    match lhs {
                        Expression::Variable(name) => {
                            lhs = self.function_call_expression(&name)?;
                            token = self.peek();
                        }
                        _ => return Err(ParsingError::InvalidCall(*token.source())),
                    }
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
                    .expression_bp(right_binding)
                    .map_err(|_| ParsingError::MissingOperand(*token.source()))?;

                lhs = Expression::binary(op, lhs, rhs);

                continue;
            }

            break;
        }

        Ok(lhs)
    }

    fn variable_expression(&mut self, name: &str) -> Result<Expression, ParsingError> {
        trace!("Parsing variable expression (name: {})", name);
        Ok(Expression::Variable(name.to_string()))
    }

    fn function_call_expression(&mut self, name: &str) -> Result<Expression, ParsingError> {
        trace!("Parsing function call expression (name: {})", name);
        let mut arguments = vec![];
        self.consume(Token::LeftParen)?;
        if let Some(Token::RightParen) = self.tokens.peek().map(|t| t.kind()) {
            self.consume(Token::RightParen)?;
            return Ok(Expression::Call(name.to_string(), arguments));
        }
        loop {
            let expr = self.expression_bp(0)?;
            arguments.push(expr);
            let token = self.advance();
            match token.kind() {
                Token::Comma => continue,
                Token::RightParen => break,
                _ => return Err(ParsingError::MissingClosingParentheses(*token.source())),
            }
        }
        Ok(Expression::Call(name.to_string(), arguments))
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
            Token::Equal => Some(BinaryOperator::Assign),
            _ => None,
        }
    }

    fn infix_binding(&self, token: &SourceToken) -> Option<(u8, u8)> {
        match token.kind() {
            Token::Plus | Token::Minus => Precedence::Term.infix_binding(),
            Token::Star | Token::Slash => Precedence::Factor.infix_binding(),
            Token::EqualEqual | Token::BangEqual => Precedence::Equality.infix_binding(),
            Token::Less | Token::LessEqual => Precedence::Comparison.infix_binding(),
            Token::Greater | Token::GreaterEqual => Precedence::Comparison.infix_binding(),
            Token::Equal => Precedence::Assignment.infix_binding(),
            _ => None,
        }
    }

    fn postfix_binding(&self, token: &SourceToken) -> Option<u8> {
        match token.kind() {
            Token::LeftSquare => Precedence::Index.postfix_binding(),
            Token::LeftParen => Precedence::Call.postfix_binding(),
            _ => None,
        }
    }

    fn prefix_binding(&self, token: SourceToken) -> Option<u8> {
        match token.kind() {
            Token::Minus | Token::Bang => Precedence::Unary.prefix_binding(),
            _ => None,
        }
    }
}

enum Precedence {
    None,
    Assignment,
    Or,
    And,
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
            Precedence::None => 0,
            Precedence::Assignment => 1,
            Precedence::Or => 3,
            Precedence::And => 5,
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
            Precedence::None | Precedence::Unary | Precedence::Index => None,
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
    fn test_expression() {
        let mut parser = Parser::new(Lexer::new("1 + 2 * 3"));
        let expr = parser.expression().unwrap();
        assert_eq!(
            expr,
            Expression::binary(
                BinaryOperator::Add,
                Expression::number(1),
                Expression::binary(
                    BinaryOperator::Mul,
                    Expression::number(2),
                    Expression::number(3)
                )
            )
        );
    }

    #[test]
    fn test_expression_with_parentheses() {
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
    fn test_expression_with_unary_operator() {
        let mut parser = Parser::new(Lexer::new("-1 + 2"));
        let expr = parser.expression().unwrap();
        assert_eq!(
            expr,
            Expression::binary(
                BinaryOperator::Add,
                Expression::unary(UnaryOperator::Negate, Expression::number(1)),
                Expression::number(2)
            )
        );
    }

    #[test]
    fn assignment_expression() {
        let mut parser = Parser::new(Lexer::new("a = 1"));
        let expr = parser.expression().unwrap();
        assert_eq!(
            expr,
            Expression::binary(
                BinaryOperator::Assign,
                Expression::Variable("a".to_string()),
                Expression::number(1)
            )
        );
    }

    #[test]
    fn test_expression_with_function_call() {
        let mut parser = Parser::new(Lexer::new("foo()"));
        let expr = parser.expression().unwrap();
        assert_eq!(expr, Expression::Call("foo".to_string(), vec![]));
    }

    #[test]
    fn test_expression_with_function_call_with_arguments() {
        let mut parser = Parser::new(Lexer::new("foo(1, 2)"));
        let expr = parser.expression().unwrap();
        assert_eq!(
            expr,
            Expression::Call(
                "foo".to_string(),
                vec![Expression::number(1), Expression::number(2)]
            )
        );
    }
}
