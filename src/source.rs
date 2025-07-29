//! Keeps track of source file positions in interpreter.

use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Position {
    line: usize,
    column: usize,
}

impl Position {
    pub fn new(line: usize, column: usize) -> Self {
        Position { line, column }
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn column(&self) -> usize {
        self.column
    }
}

impl From<(usize, usize)> for Position {
    fn from((line, column): (usize, usize)) -> Self {
        Position { line, column }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}:{}]", self.line, self.column)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position() {
        let pos = Position::new(1, 2);
        assert_eq!(pos.line(), 1);
        assert_eq!(pos.column(), 2);
        assert_eq!(format!("{pos}"), "[1:2]");
    }
}
