//! Compiles AST into virtual machine instructions
use crate::ast::{Expression, Operation, Program, Statement};
use crate::vm::opcode::{Chunk, Op};

#[derive(Debug, Clone, Default)]
pub struct Compiler {
    chunk: Chunk,
}

impl Compiler {
    pub fn compile_program(&mut self, program: Program) -> Chunk {
        for statement in program.statements() {
            self.statement(statement);
        }
        self.chunk.clone()
    }

    pub fn compile(&mut self, ast: &Statement) -> Chunk {
        self.statement(ast);
        self.chunk.clone()
    }

    fn statement(&mut self, ast: &Statement) {
        match ast {
            Statement::Expression(expr) => self.expression(expr),
            Statement::Print(expr) => {
                self.expression(expr);
                self.chunk.add(Op::Print);
            }
            Statement::Declaration(name, value) => {
                match value {
                    Some(expr) => self.expression(expr),
                    None => self.chunk.add(Op::Nil),
                }
                let variable_name = name.clone();
                self.chunk.add(Op::Global(variable_name));
            }
            Statement::Assignment(name, expr) => {
                self.expression(expr);
                let variable_name = name.clone();
                self.chunk.add(Op::Global(variable_name));
            }
        }
    }

    fn expression(&mut self, ast: &Expression) {
        match ast {
            Expression::NumberLiteral(n) => self.chunk.add(Op::LoadFloat(*n)),
            Expression::BinaryOperation(op, a, b) => {
                self.expression(b);
                self.expression(a);
                match op {
                    Operation::Add => self.chunk.add(Op::Add),
                    Operation::Sub => self.chunk.add(Op::Sub),
                    Operation::Mul => self.chunk.add(Op::Mul),
                    Operation::Div => self.chunk.add(Op::Div),
                }
            }
            Expression::UnaryOperation(Operation::Sub, lhs) => {
                self.expression(lhs);
                self.chunk.add(Op::Neg)
            }
            Expression::UnaryOperation(op, _) => {
                panic!("unsupported unary operation {:?}", op);
            }
            Expression::Cmp(a, b) => {
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
        let number = Statement::expression(Expression::number(42.0));
        let mut compiler = Compiler::default();

        let chunk = compiler.compile(&number);

        assert_eq!(chunk.op(0), Some(&Op::LoadFloat(42.0)));
    }

    #[test]
    fn compile_arithmetic_expressions() {
        let add_expression = Expression::BinaryOperation(
            Operation::Add,
            Box::new(Expression::NumberLiteral(3.0)),
            Box::new(Expression::NumberLiteral(8.5)),
        );
        let add_statement = Statement::expression(add_expression.clone());
        let mut compiler = Compiler::default();

        let chunk: Chunk = compiler.compile(&add_statement);

        assert_eq!(chunk.op(0), Some(&Op::LoadFloat(8.5)));
        assert_eq!(chunk.op(1), Some(&Op::LoadFloat(3.0)));
        assert_eq!(chunk.op(2), Some(&Op::Add));
    }
}
