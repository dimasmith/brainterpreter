//! Compiles AST into virtual machine instructions
use crate::ast::{Expression, Operation, Program, Statement};
use crate::vm::opcode::{Chunk, Op};
use thiserror::Error;

type CompilationResult = Result<(), CompileError>;

#[derive(Debug, Clone, Default)]
pub struct Compiler {
    chunk: Chunk,
    locals: Locals,
}

#[derive(Debug, Clone, PartialEq, Error)]
pub enum CompileError {
    #[error("compilation failed")]
    Unknown,
    #[error("variable {0} is already declared in this scope")]
    VariableAlreadyDeclared(String),
}

/// Represents a local variable in the current scope
#[derive(Debug, Clone)]
struct Local {
    name: String,
    depth: usize,
    initialized: bool,
}

/// Contains local variables
#[derive(Debug, Clone, Default)]
struct Locals {
    locals: Vec<Local>,
    depth: usize,
}

impl Compiler {
    pub fn compile_program(&mut self, program: Program) -> Result<Chunk, CompileError> {
        for statement in program.statements() {
            self.statement(statement)?;
        }
        Ok(self.chunk.clone())
    }

    fn statement(&mut self, ast: &Statement) -> CompilationResult {
        match ast {
            Statement::Expression(expr) => self.expression(expr),
            Statement::Print(expr) => {
                self.expression(expr)?;
                self.chunk.add(Op::Print);
                Ok(())
            }
            Statement::Declaration(name, value) => self.variable_declaration(name, value),
            Statement::Assignment(name, expr) => self.variable_assignment(name, expr),
            Statement::Block(statements) => {
                self.block(statements)?;
                Ok(())
            }
        }
    }

    fn variable_assignment(
        &mut self,
        name: &String,
        expr: &Expression,
    ) -> Result<(), CompileError> {
        if self.locals.depth > 0 {
            if let Some(local) = self.locals.resolve_local(name) {
                self.expression(expr)?;
                self.chunk.add(Op::WriteLocal(local));
                return Ok(());
            }
        }
        self.expression(expr)?;
        let variable_name = name.clone();
        self.chunk.add(Op::Global(variable_name));
        Ok(())
    }

    fn variable_declaration(
        &mut self,
        name: &str,
        value: &Option<Expression>,
    ) -> CompilationResult {
        if self.locals.depth > 0 {
            if self.locals.check_local(name) {
                return Err(CompileError::VariableAlreadyDeclared(name.to_string()));
            }
            self.locals.add_local(name);
            if let Some(value) = value {
                self.expression(value)?;
            } else {
                self.chunk.add(Op::Nil);
            }
            self.locals.initialize_last_local();
            self.chunk.add(Op::WriteLocal(self.locals.locals.len() - 1));
            return Ok(());
        }

        match value {
            Some(expr) => self.expression(expr)?,
            None => self.chunk.add(Op::Nil),
        }
        self.chunk.add(Op::Global(name.to_string()));
        Ok(())
    }

    fn expression(&mut self, ast: &Expression) -> CompilationResult {
        match ast {
            Expression::NumberLiteral(n) => self.chunk.add(Op::LoadFloat(*n)),
            Expression::BooleanLiteral(b) => self.chunk.add(Op::LoadBool(*b)),
            Expression::BinaryOperation(op, a, b) => {
                self.expression(b)?;
                self.expression(a)?;
                match op {
                    Operation::Add => self.chunk.add(Op::Add),
                    Operation::Sub => self.chunk.add(Op::Sub),
                    Operation::Mul => self.chunk.add(Op::Mul),
                    Operation::Div => self.chunk.add(Op::Div),
                    Operation::Equal => self.chunk.add(Op::Cmp),
                    Operation::NotEqual => {
                        self.chunk.add(Op::Cmp);
                        self.chunk.add(Op::Not)
                    }
                    Operation::Less => {
                        self.chunk.add(Op::Ge);
                        self.chunk.add(Op::Not)
                    }
                    Operation::Greater => {
                        self.chunk.add(Op::Le);
                        self.chunk.add(Op::Not)
                    }
                    Operation::LessOrEqual => self.chunk.add(Op::Le),
                    Operation::GreaterOrEqual => self.chunk.add(Op::Ge),
                    Operation::Not => panic!("not is not a binary operation"),
                }
            }
            Expression::Variable(name) => self.load_variable(name),
            Expression::UnaryOperation(Operation::Sub, lhs) => {
                self.expression(lhs)?;
                self.chunk.add(Op::LoadFloat(0.0));
                self.chunk.add(Op::Sub)
            }
            Expression::UnaryOperation(Operation::Not, lhs) => {
                self.expression(lhs)?;
                self.chunk.add(Op::Not)
            }
            Expression::UnaryOperation(op, _) => {
                panic!("unsupported unary operation {:?}", op);
            }
            Expression::Cmp(a, b) => {
                self.expression(b)?;
                self.expression(a)?;
                self.chunk.add(Op::Cmp);
            }
        }
        Ok(())
    }

    fn load_variable(&mut self, name: &str) {
        if let Some(local) = self.locals.resolve_local(name) {
            self.chunk.add(Op::ReadLocal(local));
            return;
        }
        self.chunk.add(Op::LoadGlobal(name.to_string()));
    }

    fn block(&mut self, statements: &Vec<Statement>) -> CompilationResult {
        self.begin_scope();
        for statement in statements {
            self.statement(statement)?;
        }
        self.end_scope();
        Ok(())
    }

    fn begin_scope(&mut self) {
        self.locals.begin_scope();
    }

    fn end_scope(&mut self) {
        let locals_in_scope = self.locals.end_scope();
        for _ in 0..locals_in_scope {
            self.chunk.add(Op::Pop);
        }
    }
}

impl Locals {
    /// Find the index of a local variable by name.
    fn resolve_local(&self, name: &str) -> Option<usize> {
        for (i, local) in self.locals.iter().enumerate().rev() {
            if local.name == *name && local.initialized {
                return Some(i);
            }
        }
        None
    }

    fn check_local_on_depth(&self, name: &str, depth: usize) -> bool {
        if let Some(offset) = self.resolve_local(name) {
            return self.locals[offset].depth == depth;
        }
        false
    }

    fn check_local(&self, name: &str) -> bool {
        self.check_local_on_depth(name, self.depth)
    }

    fn add_local(&mut self, name: &str) -> Local {
        let local = Local {
            name: name.to_string(),
            depth: self.depth,
            initialized: false,
        };
        self.locals.push(local.clone());
        local
    }

    fn begin_scope(&mut self) {
        self.depth += 1;
    }

    fn end_scope(&mut self) -> usize {
        let locals_in_scope = self
            .locals
            .iter()
            .rev()
            .take_while(|local| local.depth == self.depth)
            .count();
        for _ in 0..locals_in_scope {
            self.locals.pop();
        }
        self.depth -= 1;
        locals_in_scope
    }

    fn initialize_last_local(&mut self) {
        self.locals.last_mut().unwrap().initialized = true;
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn compile_number_literal() {
        let number = Statement::expression(Expression::number(42.0));
        let mut compiler = Compiler::default();

        let chunk = compiler
            .compile_program(Program::new(vec![number]))
            .unwrap();

        assert_eq!(chunk.op(0), Some(&Op::LoadFloat(42.0)));
    }

    #[test]
    fn compile_arithmetic_expressions() {
        let add_expression = Expression::BinaryOperation(
            Operation::Add,
            Box::new(Expression::NumberLiteral(3.0)),
            Box::new(Expression::NumberLiteral(8.5)),
        );
        let add_statement = Statement::expression(add_expression);
        let mut compiler = Compiler::default();

        let chunk: Chunk = compiler
            .compile_program(Program::new(vec![add_statement]))
            .unwrap();

        assert_eq!(chunk.op(0), Some(&Op::LoadFloat(8.5)));
        assert_eq!(chunk.op(1), Some(&Op::LoadFloat(3.0)));
        assert_eq!(chunk.op(2), Some(&Op::Add));
    }

    #[test]
    fn compile_locals() {
        let block_assignments = vec![
            Statement::Declaration("a".to_string(), Some(Expression::number(1.0))),
            Statement::Declaration("b".to_string(), Some(Expression::number(2.0))),
        ];
        let block = Statement::Block(block_assignments);
        let mut compiler = Compiler::default();

        let program = compiler.compile_program(Program::new(vec![block])).unwrap();

        let opcodes: Vec<Op> = program.iter().cloned().collect();
        assert_eq!(
            opcodes,
            vec![
                Op::LoadFloat(1.0),
                Op::WriteLocal(0),
                Op::LoadFloat(2.0),
                Op::WriteLocal(1),
                Op::Pop,
                Op::Pop,
            ]
        );
    }

    #[test]
    fn shadow_initialization() {
        let global = Statement::Declaration("a".to_string(), Some(Expression::number(1.0)));
        let local =
            Statement::Declaration("a".to_string(), Some(Expression::Variable("a".to_string())));
        let block = Statement::Block(vec![local]);
        let mut compiler = Compiler::default();

        let chunk = compiler
            .compile_program(Program::new(vec![global, block]))
            .unwrap();
        let opcodes: Vec<Op> = chunk.iter().cloned().collect();

        assert_eq!(
            opcodes,
            vec![
                Op::LoadFloat(1.0),
                Op::Global("a".to_string()),
                Op::LoadGlobal("a".to_string()),
                Op::WriteLocal(0),
                Op::Pop,
            ]
        );
    }

    mod locals {
        use super::*;

        #[test]
        fn add_local() {
            let mut locals = Locals::default();
            locals.begin_scope();
            let local = locals.add_local("a");
            assert_eq!(local.depth, 1);

            locals.begin_scope();
            let local = locals.add_local("b");
            assert_eq!(local.depth, 2);

            locals.end_scope();
            let local = locals.add_local("c");
            assert_eq!(local.depth, 1);

            locals.end_scope();
        }

        #[test]
        fn begin_scope() {
            let mut locals = Locals::default();
            locals.begin_scope();
            assert_eq!(locals.depth, 1);

            locals.begin_scope();
            assert_eq!(locals.depth, 2);
        }

        #[test]
        fn end_scope() {
            let mut locals = Locals::default();
            locals.begin_scope(); // outer scope
            locals.begin_scope(); // inner scope
            locals.add_local("a");
            locals.add_local("b");
            let locals_in_scope = locals.end_scope();
            assert_eq!(locals_in_scope, 2, "inner scope had 2 variables");
            assert_eq!(locals.depth, 1, "inner scope ended");
            let locals_in_scope = locals.end_scope();
            assert_eq!(locals_in_scope, 0, "outer scope had no variables");
            assert_eq!(locals.depth, 0, "outer scope ended");
        }

        #[test]
        fn resolve_locals() {
            let mut locals = Locals::default();
            locals.begin_scope();
            locals.add_local("a");
            locals.initialize_last_local();
            locals.begin_scope();
            locals.add_local("b");
            locals.initialize_last_local();

            assert_eq!(locals.resolve_local("a"), Some(0));
            assert_eq!(locals.resolve_local("b"), Some(1));
            assert_eq!(locals.resolve_local("c"), None);

            locals.end_scope();
            assert_eq!(
                locals.resolve_local("a"),
                Some(0),
                "a should still be in scope"
            );
            assert_eq!(locals.resolve_local("b"), None, "b should not be in scope");
        }

        #[test]
        fn check_local_on_depth_level() {
            let mut locals = Locals::default();
            locals.begin_scope();
            locals.add_local("a");
            locals.initialize_last_local();
            locals.begin_scope();
            locals.add_local("b");
            locals.initialize_last_local();

            assert!(
                locals.check_local_on_depth("a", 1),
                "a should be on depth 1"
            );
            assert!(
                !locals.check_local_on_depth("a", 2),
                "a should not be on depth 2"
            );
            assert!(
                !locals.check_local_on_depth("b", 1),
                "b should not be on depth 1"
            );
            assert!(
                locals.check_local_on_depth("b", 2),
                "b should be on depth 2"
            );
            assert!(
                !locals.check_local_on_depth("c", 2),
                "c should not be on any level"
            );
        }
    }
}
