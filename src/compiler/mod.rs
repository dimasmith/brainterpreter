//! Compiles AST into virtual machine instructions
use log::trace;
use std::rc::Rc;
use thiserror::Error;

use locals::Locals;

use crate::ast::{BinaryOperator, Expression, Program, Statement, UnaryOperator};
use crate::value::{Function, ValueType};
use crate::vm::exec::Chunk;
use crate::vm::opcode::Op;

use self::chunk::ChunkBuilder;

pub mod chunk;
mod locals;

type CompilationResult = Result<(), CompileError>;

#[derive(Debug, Clone, Default)]
pub struct Compiler {
    chunk: ChunkBuilder,
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
    pub fn compile(&mut self, program: Program) -> Result<Chunk, CompileError> {
        // TODO: this delegation approach is weird. Get rid of it.
        let script_compiler = Compiler::default();
        let chunk_builder = script_compiler.compile_part(program)?;
        Ok(chunk_builder.build())
    }

    fn compile_part(mut self, program: Program) -> Result<ChunkBuilder, CompileError> {
        for statement in program.statements() {
            self.statement(statement)?;
        }
        Ok(self.chunk)
    }

    fn statement(&mut self, ast: &Statement) -> CompilationResult {
        trace!("Compiling statement: {ast:?}");
        match ast {
            Statement::Expression(expr) => self.expression_statement(expr),
            Statement::Print(expr) => self.print_statement(expr),
            Statement::DeclareVariable(name) => self.declare_variable(name),
            Statement::DefineVariable(name, value) => self.define_variable(name, value),
            Statement::Block(statements) => self.block_statement(statements),
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

    fn block_statement(&mut self, statements: &Vec<Statement>) -> Result<(), CompileError> {
        self.block(statements)?;
        Ok(())
    }

    fn print_statement(&mut self, expr: &Expression) -> Result<(), CompileError> {
        self.expression(expr)?;
        self.chunk.add_op(Op::Print);
        Ok(())
    }

    fn assign_variable(&mut self, name: &str, value: &Expression) -> Result<(), CompileError> {
        self.expression(value)?;
        if self.locals.depth() > 0 {
            if let Some(local) = self.locals.resolve_local(name) {
                self.chunk.add_op(Op::StoreLocal(local));
                return Ok(());
            }
        }
        self.store_global(name);
        Ok(())
    }

    fn assign_variable_from_stack(&mut self, name: &str) -> Result<(), CompileError> {
        if self.locals.depth() > 0 {
            if let Some(local) = self.locals.resolve_local(name) {
                self.chunk.add_op(Op::StoreLocal(local));
                return Ok(());
            }
        }
        self.store_global(name);
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
        self.chunk.add_op(Op::Nil);
        self.store_global(name);
        Ok(())
    }

    fn define_variable(&mut self, name: &str, value: &Expression) -> CompilationResult {
        if self.locals.depth() > 0 {
            if self.locals.check_local(name) {
                return Err(CompileError::VariableAlreadyDeclared(name.to_string()));
            }
            self.locals.add_local(name);
            self.expression(value)?;
            self.locals.initialize_last_local();
            self.chunk.add_op(Op::StoreLocal(self.locals.last_index()));
            return Ok(());
        }

        self.expression(value)?;
        self.store_global(name);
        self.chunk.add_op(Op::Pop);
        Ok(())
    }

    fn expression_statement(&mut self, expr: &Expression) -> CompilationResult {
        self.expression(expr)?;
        self.chunk.add_op(Op::Pop);
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
            Expression::AssignIndexVariable {
                variable,
                index,
                value,
            } => {
                self.assign_index_variable(variable, index, value)?;
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
                }
            }
            Expression::Variable(name) => self.load_variable(name),
            Expression::FunctionCall(name, args) => self.function_call(name, args)?,
            Expression::UnaryOperation(UnaryOperator::Negate, lhs) => {
                self.expression(lhs)?;
                self.chunk.add_op(Op::ConstFloat(0.0));
                self.chunk.add_op(Op::Sub);
            }
            Expression::UnaryOperation(UnaryOperator::Not, lhs) => {
                self.expression(lhs)?;
                self.chunk.add_op(Op::Not);
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

    fn assign_index_variable(
        &mut self,
        variable: &str,
        index: &Expression,
        value: &Expression,
    ) -> CompilationResult {
        self.expression(index)?;
        self.load_variable(variable);
        self.expression(value)?;
        self.chunk.add_op(Op::StoreIndex);
        self.assign_variable_from_stack(variable)?;
        Ok(())
    }

    fn load_variable(&mut self, name: &str) {
        if let Some(local) = self.locals.resolve_local(name) {
            self.chunk.add_op(Op::LoadLocal(local));
            return;
        }
        self.load_global(name);
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
            let jump_offset = self.chunk.last_op_address() - then_jump;
            self.chunk.patch_jump(then_jump, jump_offset as i32);
            self.statement(otherwise)?;
            self.chunk.patch_jump_to_last(else_jump);
        } else {
            self.chunk.patch_jump_to_last(then_jump);
        }
        Ok(())
    }

    fn while_statement(&mut self, condition: &Expression, body: &Statement) -> CompilationResult {
        let loop_start = self.chunk.last_op_address();
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
        body: &Statement,
    ) -> CompilationResult {
        let mut function_compiler = Compiler::default();
        function_compiler.begin_scope();
        for param in params {
            function_compiler.declare_variable(param)?;
        }
        let function_program = Program::new(vec![body.clone()]);
        let mut chunk_builder = function_compiler.compile_part(function_program)?;
        chunk_builder.add_op(Op::Nil);
        chunk_builder.add_op(Op::Return);
        let chunk = Rc::new(chunk_builder.build());
        let function = Function::new(name.to_string(), Rc::clone(&chunk), params.len());
        let n = self
            .chunk
            .add_constant(ValueType::Function(Box::new(function)));
        self.chunk.add_op(Op::Const(n));
        self.store_global(name);
        self.chunk.add_op(Op::Pop);
        Ok(())
    }

    fn function_call(&mut self, name: &str, args: &Vec<Expression>) -> CompilationResult {
        self.load_global(name);
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

    fn load_global(&mut self, name: &str) {
        let const_idx = self.chunk.add_constant(ValueType::string(name));
        self.chunk.add_op(Op::LoadGlobal(const_idx));
    }

    fn store_global(&mut self, name: &str) {
        let const_idx = self.chunk.add_constant(ValueType::string(name));
        self.chunk.add_op(Op::StoreGlobal(const_idx));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assign_global_variable() {
        let assign = Statement::Expression(Expression::AssignVariable(
            "a".to_string(),
            Box::new(Expression::number(42)),
        ));
        let program = Program::new(vec![assign]);
        let mut compiler = Compiler::default();

        let chunk = compiler.compile(program).unwrap();
        let ops: Vec<&Op> = chunk.ops().collect();

        assert_eq!(
            ops,
            vec![&Op::ConstFloat(42.0), &Op::StoreGlobal(0), &Op::Pop]
        );
    }

    #[test]
    fn compile_number_literal() {
        let number = Statement::expression(Expression::number(42.0));
        let mut compiler = Compiler::default();

        let chunk = compiler.compile(Program::new(vec![number])).unwrap();

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

        let chunk: Chunk = compiler.compile(Program::new(vec![add_statement])).unwrap();

        assert_eq!(chunk.op(0), Some(&Op::ConstFloat(8.5)));
        assert_eq!(chunk.op(1), Some(&Op::ConstFloat(3.0)));
        assert_eq!(chunk.op(2), Some(&Op::Add));
    }

    #[test]
    fn compile_locals() {
        let block_assignments = vec![
            Statement::DefineVariable("a".to_string(), Expression::number(1.0)),
            Statement::DefineVariable("b".to_string(), Expression::number(2.0)),
        ];
        let block = Statement::Block(block_assignments);
        let mut compiler = Compiler::default();

        let program = compiler.compile(Program::new(vec![block])).unwrap();

        let opcodes: Vec<Op> = program.ops().cloned().collect();
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
        let global = Statement::DefineVariable("a".to_string(), Expression::number(1.0));
        let local =
            Statement::DefineVariable("a".to_string(), Expression::Variable("a".to_string()));
        let block = Statement::Block(vec![local]);
        let mut compiler = Compiler::default();

        let chunk = compiler.compile(Program::new(vec![global, block])).unwrap();
        let opcodes: Vec<Op> = chunk.ops().cloned().collect();

        assert_eq!(
            opcodes,
            vec![
                Op::ConstFloat(1.0),
                Op::StoreGlobal(0),
                Op::Pop,
                Op::LoadGlobal(0),
                Op::StoreLocal(0),
                Op::Pop,
            ]
        );
    }
}
