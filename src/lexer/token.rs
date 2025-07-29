use std::fmt::{Display, Formatter};

use crate::lexer::SourceToken;
use crate::source::Position;

/// Lexical token
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Plus,
    Minus,
    Star,
    Slash,
    DoubleQuote,
    LeftParen,
    RightParen,
    LeftCurly,
    RightCurly,
    LeftSquare,
    RightSquare,
    Equal,
    EqualEqual,
    Bang,
    BangEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    Semicolon,
    Comma,
    Number(f64),
    True,
    False,
    Print,
    If,
    Else,
    While,
    Let,
    Fun,
    Return,
    Nil,
    Identifier(String),
    StringLiteral(String),
    EndOfFile,
    Error,
}

impl Token {
    pub fn with_position(self, pos: Position) -> SourceToken {
        SourceToken::new(self, pos)
    }

    pub fn with_line(self, line: usize) -> SourceToken {
        SourceToken::new(self, Position::new(line, 0))
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Star => write!(f, "*"),
            Token::Slash => write!(f, "/"),
            Token::DoubleQuote => write!(f, "\""),
            Token::LeftParen => write!(f, "("),
            Token::RightParen => write!(f, ")"),
            Token::LeftCurly => write!(f, "{{"),
            Token::RightCurly => write!(f, "}}"),
            Token::LeftSquare => write!(f, "["),
            Token::RightSquare => write!(f, "]"),
            Token::Equal => write!(f, "="),
            Token::EqualEqual => write!(f, "=="),
            Token::Bang => write!(f, "!"),
            Token::BangEqual => write!(f, "!="),
            Token::Less => write!(f, "<"),
            Token::Greater => write!(f, ">"),
            Token::LessEqual => write!(f, "<="),
            Token::GreaterEqual => write!(f, ">="),
            Token::Semicolon => write!(f, ";"),
            Token::Comma => write!(f, ","),
            Token::Number(n) => write!(f, "{n}"),
            Token::True => write!(f, "true"),
            Token::False => write!(f, "false"),
            Token::Print => write!(f, "print"),
            Token::If => write!(f, "if"),
            Token::Else => write!(f, "else"),
            Token::While => write!(f, "while"),
            Token::Let => write!(f, "let"),
            Token::Fun => write!(f, "fun"),
            Token::Return => write!(f, "return"),
            Token::Nil => write!(f, "nil"),
            Token::Identifier(name) => write!(f, "{name}"),
            Token::StringLiteral(s) => write!(f, "{s}"),
            Token::EndOfFile => write!(f, "EOF"),
            Token::Error => write!(f, "Error"),
        }
    }
}
