//! Compiles AST into virtual machine instructions
use log::trace;
use thiserror::Error;

use locals::Locals;

use crate::ast::{BinaryOperator, Expression, Program, Statement, UnaryOperator};
use crate::value::{Function, ValueType};
use crate::vm::opcode::{Chunk, Op};

mod locals;

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
    #[error("unsupported assignment target: {context}")]
    UnsupportedAssignmentTarget { context: String },
}

impl Compiler {
    pub fn compile_script(&mut self, program: Program) -> Result<Function, CompileError> {
        let mut script_compiler = Compiler::default();
        let chunk = script_compiler.compile_program(program)?;
        let script = Function::script(chunk);
        self.chunk
            .add_constant(ValueType::Function(Box::new(script.clone())));
        self.chunk.add_op(Op::Call(0));
        Ok(script)
    }

    pub fn compile_program(&mut self, program: Program) -> Result<Chunk, CompileError> {
        for statement in program.statements() {
            self.statement(statement)?;
        }
        Ok(self.chunk.clone())
    }

    fn statement(&mut self, ast: &Statement) -> CompilationResult {
        trace!("Compiling statement: {:?}", ast);
        match ast {
            Statement::Expression(expr) => self.expression(expr),
            Statement::Print(expr) => {
                self.expression(expr)?;
                self.chunk.add_op(Op::Print);
                Ok(())
            }
            Statement::Variable(name, value) => self.variable_declaration(name, value),
            Statement::Block(statements) => {
                self.block(statements)?;
                Ok(())
            }
            Statement::If(condition, then, otherwise) => {
                self.if_statement(condition, then, otherwise)
            }
            Statement::While(condition, body) => self.while_statement(condition, body),
            Statement::Function(name, params, body) => {
                self.function_declaration(name, params, body)
            }
            Statement::Return(expr) => self.return_statement(expr),
        }
    }

    fn assign_variable(&mut self, name: &str, expr: &Expression) -> Result<(), CompileError> {
        if self.locals.depth() > 0 {
            if let Some(local) = self.locals.resolve_local(name) {
                self.expression(expr)?;
                self.chunk.add_op(Op::StoreLocal(local));
                self.chunk.add_op(Op::Pop); // removing hanging expression result from stack
                return Ok(());
            }
        }
        self.expression(expr)?;
        self.chunk.add_op(Op::StoreGlobal(name.to_string()));
        Ok(())
    }

    fn assign_variable_from_stack(&mut self, name: &str) -> Result<(), CompileError> {
        if self.locals.depth() > 0 {
            if let Some(local) = self.locals.resolve_local(name) {
                self.chunk.add_op(Op::StoreLocal(local));
                self.chunk.add_op(Op::Pop); // removing hanging expression result from stack
                return Ok(());
            }
        }
        self.chunk.add_op(Op::StoreGlobal(name.to_string()));
        Ok(())
    }

    fn variable_declaration(
        &mut self,
        name: &str,
        value: &Option<Expression>,
    ) -> CompilationResult {
        if self.locals.depth() > 0 {
            if self.locals.check_local(name) {
                return Err(CompileError::VariableAlreadyDeclared(name.to_string()));
            }
            self.locals.add_local(name);
            if let Some(value) = value {
                self.expression(value)?;
            } else {
                self.chunk.add_op(Op::Nil);
            }
            self.locals.initialize_last_local();
            self.chunk.add_op(Op::StoreLocal(self.locals.last_index()));
            return Ok(());
        }

        match value {
            Some(expr) => self.expression(expr)?,
            None => {
                self.chunk.add_op(Op::Nil);
            }
        }
        self.chunk.add_op(Op::StoreGlobal(name.to_string()));
        Ok(())
    }

    fn declare_variable(&mut self, name: &str) -> CompilationResult {
        if self.locals.depth() > 0 {
            if self.locals.check_local(name) {
                return Err(CompileError::VariableAlreadyDeclared(name.to_string()));
            }
            self.locals.add_local(name);
            self.locals.initialize_last_local();
            return Ok(());
        }
        Ok(())
    }

    fn expression(&mut self, ast: &Expression) -> CompilationResult {
        match ast {
            Expression::Nil => {
                self.chunk.add_op(Op::Nil);
            }
            Expression::NumberLiteral(n) => {
                self.chunk.add_op(Op::ConstFloat(*n));
            }
            Expression::StringLiteral(s) => {
                let n = self
                    .chunk
                    .add_constant(ValueType::Text(Box::new(s.clone())));
                self.chunk.add_op(Op::Const(n));
            }
            Expression::BooleanLiteral(b) => {
                self.chunk.add_op(Op::ConstBool(*b));
            }
            Expression::AssignVariable(name, expr) => {
                self.assign_variable(name, expr)?;
            }
            Expression::Assign { target, value } => {
                self.assign(target, value)?;
            }
            Expression::Array { initial, size } => self.initialize_array(initial, size)?,
            Expression::BinaryOperation(op, a, b) => {
                self.expression(b)?;
                self.expression(a)?;
                match op {
                    BinaryOperator::Add => {
                        self.chunk.add_op(Op::Add);
                    }
                    BinaryOperator::Sub => {
                        self.chunk.add_op(Op::Sub);
                    }
                    BinaryOperator::Mul => {
                        self.chunk.add_op(Op::Mul);
                    }
                    BinaryOperator::Div => {
                        self.chunk.add_op(Op::Div);
                    }
                    BinaryOperator::Equal => {
                        self.chunk.add_op(Op::Cmp);
                    }
                    BinaryOperator::NotEqual => {
                        self.chunk.add_op(Op::Cmp);
                        self.chunk.add_op(Op::Not);
                    }
                    BinaryOperator::Less => {
                        self.chunk.add_op(Op::Ge);
                        self.chunk.add_op(Op::Not);
                    }
                    BinaryOperator::Greater => {
                        self.chunk.add_op(Op::Le);
                        self.chunk.add_op(Op::Not);
                    }
                    BinaryOperator::LessOrEqual => {
                        self.chunk.add_op(Op::Le);
                    }
                    BinaryOperator::GreaterOrEqual => {
                        self.chunk.add_op(Op::Ge);
                    }
                    BinaryOperator::Assign => {
                        // everything is already on the stack
                    }
                }
            }
            Expression::Variable(name) => self.load_variable(name),
            Expression::Call(name, args) => self.function_call(name, args)?,
            Expression::UnaryOperation(UnaryOperator::Negate, lhs) => {
                self.expression(lhs)?;
                self.chunk.add_op(Op::ConstFloat(0.0));
                self.chunk.add_op(Op::Sub);
            }
            Expression::UnaryOperation(UnaryOperator::Not, lhs) => {
                self.expression(lhs)?;
                self.chunk.add_op(Op::Not);
            }
            Expression::Cmp(a, b) => {
                self.expression(b)?;
                self.expression(a)?;
                self.chunk.add_op(Op::Cmp);
            }
            Expression::Index { array, index } => {
                self.expression(index)?;
                self.expression(array)?;
                self.chunk.add_op(Op::LoadIndex);
            }
        }
        Ok(())
    }

    fn initialize_array(&mut self, initial: &Expression, size: &Expression) -> CompilationResult {
        self.expression(size)?;
        self.expression(initial)?;
        self.chunk.add_op(Op::Array);

        Ok(())
    }

    fn assign(&mut self, target: &Expression, value: &Expression) -> CompilationResult {
        match target {
            Expression::Variable(name) => self.assign_variable(name, value),
            Expression::Index { array, index } => self.assign_index(array, index, value),
            _ => Err(CompileError::UnsupportedAssignmentTarget {
                context: "".to_string(),
            }),
        }
    }

    fn assign_index(
        &mut self,
        array: &Expression,
        index: &Expression,
        value: &Expression,
    ) -> CompilationResult {
        if let Expression::Variable(name) = array {
            self.expression(index)?;
            self.load_variable(name);
            self.expression(value)?;
            self.chunk.add_op(Op::StoreIndex);
            self.assign_variable_from_stack(name)?;
            Ok(())
        } else {
            Err(CompileError::UnsupportedAssignmentTarget {
                context: "".to_string(),
            })
        }
    }

    fn load_variable(&mut self, name: &str) {
        if let Some(local) = self.locals.resolve_local(name) {
            self.chunk.add_op(Op::LoadLocal(local));
            return;
        }
        self.chunk.add_op(Op::LoadGlobal(name.to_string()));
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
            self.chunk.add_op(Op::Pop);
        }
    }

    fn if_statement(
        &mut self,
        condition: &Expression,
        then: &Statement,
        otherwise: &Option<Box<Statement>>,
    ) -> CompilationResult {
        self.expression(condition)?;
        let then_jump = self.chunk.add_op(Op::JumpIfFalse(0));
        self.statement(then)?;

        if let Some(otherwise) = otherwise {
            let else_jump = self.chunk.add_op(Op::Jump(0));
            let jump_offset = self.chunk.last_index() - then_jump;
            self.chunk.patch_jump(then_jump, jump_offset as i32);
            self.statement(otherwise)?;
            self.chunk.patch_jump_to_last(else_jump);
        } else {
            self.chunk.patch_jump_to_last(then_jump);
        }
        Ok(())
    }

    fn while_statement(&mut self, condition: &Expression, body: &Statement) -> CompilationResult {
        let loop_start = self.chunk.last_index();
        self.expression(condition)?;
        let exit_jump = self.chunk.add_op(Op::JumpIfFalse(0));
        self.statement(body)?;
        let loop_jump = self.chunk.add_op(Op::Jump(0));
        self.chunk.patch_jump_to(loop_jump, loop_start);
        self.chunk.patch_jump_to_last(exit_jump);
        Ok(())
    }

    fn function_declaration(
        &mut self,
        name: &str,
        params: &Vec<String>,
        body: &[Statement],
    ) -> CompilationResult {
        let mut function_compiler = Compiler::default();
        function_compiler.begin_scope();
        for param in params {
            function_compiler.declare_variable(param)?;
        }
        let function_program = Program::new(body.to_vec());
        let mut chunk = function_compiler.compile_program(function_program)?;
        chunk.add_op(Op::Nil);
        chunk.add_op(Op::Return);
        let function = Function::new(name.to_string(), chunk, params.len());
        let n = self
            .chunk
            .add_constant(ValueType::Function(Box::new(function)));
        self.chunk.add_op(Op::Const(n));
        self.chunk.add_op(Op::StoreGlobal(name.to_string()));
        Ok(())
    }

    fn function_call(&mut self, name: &str, args: &Vec<Expression>) -> CompilationResult {
        self.chunk.add_op(Op::LoadGlobal(name.to_string()));
        for arg in args {
            self.expression(arg)?;
        }
        self.chunk.add_op(Op::Call(args.len()));
        Ok(())
    }

    fn return_statement(&mut self, expression: &Expression) -> CompilationResult {
        self.expression(expression)?;
        self.chunk.add_op(Op::Return);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assign_global_variable() {
        let declare = Statement::Variable("a".to_string(), None);
        let assign = Statement::Expression(Expression::AssignVariable(
            "a".to_string(),
            Box::new(Expression::number(42)),
        ));
        let program = Program::new(vec![declare, assign]);
        let mut compiler = Compiler::default();

        let script = compiler.compile_script(program).unwrap();
        let ops: Vec<&Op> = script.chunk().iter().collect();

        assert_eq!(
            ops,
            vec![
                &Op::Nil,
                &Op::StoreGlobal("a".to_string()),
                &Op::ConstFloat(42.0),
                &Op::StoreGlobal("a".to_string())
            ]
        );
    }

    #[test]
    fn compile_number_literal() {
        let number = Statement::expression(Expression::number(42.0));
        let mut compiler = Compiler::default();

        let chunk = compiler
            .compile_program(Program::new(vec![number]))
            .unwrap();

        assert_eq!(chunk.op(0), Some(&Op::ConstFloat(42.0)));
    }

    #[test]
    fn compile_arithmetic_expressions() {
        let add_expression = Expression::BinaryOperation(
            BinaryOperator::Add,
            Box::new(Expression::NumberLiteral(3.0)),
            Box::new(Expression::NumberLiteral(8.5)),
        );
        let add_statement = Statement::expression(add_expression);
        let mut compiler = Compiler::default();

        let chunk: Chunk = compiler
            .compile_program(Program::new(vec![add_statement]))
            .unwrap();

        assert_eq!(chunk.op(0), Some(&Op::ConstFloat(8.5)));
        assert_eq!(chunk.op(1), Some(&Op::ConstFloat(3.0)));
        assert_eq!(chunk.op(2), Some(&Op::Add));
    }

    #[test]
    fn compile_locals() {
        let block_assignments = vec![
            Statement::Variable("a".to_string(), Some(Expression::number(1.0))),
            Statement::Variable("b".to_string(), Some(Expression::number(2.0))),
        ];
        let block = Statement::Block(block_assignments);
        let mut compiler = Compiler::default();

        let program = compiler.compile_program(Program::new(vec![block])).unwrap();

        let opcodes: Vec<Op> = program.iter().cloned().collect();
        assert_eq!(
            opcodes,
            vec![
                Op::ConstFloat(1.0),
                Op::StoreLocal(0),
                Op::ConstFloat(2.0),
                Op::StoreLocal(1),
                Op::Pop,
                Op::Pop,
            ]
        );
    }

    #[test]
    fn shadow_initialization() {
        let global = Statement::Variable("a".to_string(), Some(Expression::number(1.0)));
        let local =
            Statement::Variable("a".to_string(), Some(Expression::Variable("a".to_string())));
        let block = Statement::Block(vec![local]);
        let mut compiler = Compiler::default();

        let chunk = compiler
            .compile_program(Program::new(vec![global, block]))
            .unwrap();
        let opcodes: Vec<Op> = chunk.iter().cloned().collect();

        assert_eq!(
            opcodes,
            vec![
                Op::ConstFloat(1.0),
                Op::StoreGlobal("a".to_string()),
                Op::LoadGlobal("a".to_string()),
                Op::StoreLocal(0),
                Op::Pop,
            ]
        );
    }
}
