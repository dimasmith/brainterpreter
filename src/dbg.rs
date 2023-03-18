//! Debug information from source files

use std::fmt::Display;
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DebugPosition {
    line: usize,
    col: usize,
}

impl From<(usize, usize)> for DebugPosition {
    fn from(value: (usize, usize)) -> Self {
        DebugPosition {
            line: value.0,
            col: value.1,
        }
    }
}

impl Default for DebugPosition {
    fn default() -> Self {
        DebugPosition { line: 1, col: 1 }
    }
}

impl Display for DebugPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}:{}]", self.line, self.col)
    }
}
