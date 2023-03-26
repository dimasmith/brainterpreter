//! Compiles AST into virtual machine instructions
use crate::ast::AstExpression;
use crate::chunk::Chunk;
use crate::ops::Op;

#[derive(Debug, Clone, Default)]
pub struct Compiler {
    chunk: Chunk,
}

impl Compiler {
    pub fn compile(&mut self, ast: &AstExpression) -> Chunk {
        self.expression(ast);
        self.chunk.clone()
    }

    fn expression(&mut self, ast: &AstExpression) {
        match ast {
            AstExpression::NumberLiteral(n) => self.chunk.add(Op::LoadFloat(*n)),
            AstExpression::Add(a, b) => {
                self.expression(b);
                self.expression(a);
                self.chunk.add(Op::Add);
            }
            AstExpression::Cmp(a, b) => {
                self.expression(b);
                self.expression(a);
                self.chunk.add(Op::Cmp);
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn compile_number_literal() {
        let number = AstExpression::NumberLiteral(42.0);
        let mut compiler = Compiler::default();

        let chunk = compiler.compile(&number);

        assert_eq!(chunk.op(0), Some(&Op::LoadFloat(42.0)));
    }

    #[test]
    fn compile_arithmetic_expressions() {
        let add_expression = AstExpression::Add(
            Box::new(AstExpression::NumberLiteral(3.0)),
            Box::new(AstExpression::NumberLiteral(8.5)),
        );
        let mut compiler = Compiler::default();

        let chunk: Chunk = compiler.compile(&add_expression);

        assert_eq!(chunk.op(0), Some(&Op::LoadFloat(8.5)));
        assert_eq!(chunk.op(1), Some(&Op::LoadFloat(3.0)));
        assert_eq!(chunk.op(2), Some(&Op::Add));
    }
}