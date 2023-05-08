//! Disassembler for the chunks of bytecode.
//!
//! It's a diagnostic tool to help find issues in compiled code.

use crate::value::ValueType;
use crate::vm::opcode::{Chunk, Op};
use std::io::{Error, Write};

/// Disassemble executable chunk into VM assembly.
pub fn disassemble(chunk: &Chunk, mut w: impl Write) -> Result<(), Error> {
    disassemble_function(chunk, "$main$", &mut w)
}

fn disassemble_function(chunk: &Chunk, name: &str, w: &mut impl Write) -> Result<(), Error> {
    let mut functions = vec![];
    writeln!(w, "fn:{}:", name)?;
    writeln!(w, "constants:")?;
    for (pos, val) in chunk.constants().iter().enumerate() {
        writeln!(w, "\t{:04x}\t{}", pos, val)?;
        if let ValueType::Function(function) = val {
            functions.push(function);
        }
    }
    writeln!(w, "code:")?;
    for (line, op) in chunk.iter().enumerate() {
        match op {
            Op::Jump(offset) | Op::JumpIfFalse(offset) => {
                let address = line.checked_add_signed(*offset as isize).unwrap();
                writeln!(w, "\t{:04x}\t{} # {:04x}", line, op, address)?;
            }
            Op::StoreGlobal(idx) | Op::LoadGlobal(idx) => {
                let var_name = chunk.constant(*idx).unwrap().as_string();
                writeln!(w, "\t{:04x}\t{} # {}", line, op, var_name)?;
            }
            o => writeln!(w, "\t{:04x}\t{}", line, o)?,
        }
    }
    writeln!(w)?;
    for function in functions.iter() {
        disassemble_function(function.chunk(), function.name(), w)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value::{Function, ValueType};
    use crate::vm::opcode::Op;
    use std::io::BufWriter;

    fn test_disassemble(chunk: &Chunk) -> String {
        let mut w = BufWriter::new(vec![]);
        disassemble(chunk, &mut w).unwrap();
        let buf = w.into_inner().unwrap();
        String::from_utf8(buf).unwrap()
    }

    #[test]
    fn disassemble_single_instruction() {
        let mut chunk = Chunk::default();
        chunk.add_op(Op::Return);

        let out = test_disassemble(&chunk);
        let mut lines = out.lines();

        assert_eq!(lines.nth(3), Some("\t0000\tRET"));
    }

    #[test]
    fn disassemble_instructions_with_parameters() {
        let mut chunk = Chunk::default();
        chunk.add_op(Op::ConstFloat(3.42));
        chunk.add_op(Op::ConstBool(true));

        let out = test_disassemble(&chunk);
        let mut lines = out.lines();

        assert_eq!(lines.nth(3), Some("\t0000\tCONST_F, 3.42"));
        assert_eq!(lines.next(), Some("\t0001\tCONST_B, true"));
    }

    #[test]
    fn disassemble_jump_instructions() {
        let mut chunk = Chunk::default();
        chunk.add_op(Op::ConstFloat(5.0));
        chunk.add_op(Op::ConstFloat(1.0));
        chunk.add_op(Op::Add);
        chunk.add_op(Op::Jump(-2));

        let out = test_disassemble(&chunk);
        let mut lines = out.lines();

        assert_eq!(lines.nth(6), Some("\t0003\tJMP, -2 # 0001"));
    }

    #[test]
    fn disassemble_string_constants() {
        let mut chunk = Chunk::default();
        chunk.add_constant(ValueType::Text(Box::new(String::from("Hello, World!"))));
        chunk.add_op(Op::Const(0));

        let out = test_disassemble(&chunk);
        let mut lines = out.lines();

        assert_eq!(lines.nth(2), Some("\t0000\ts:Hello, World!"));
    }

    #[test]
    fn disassemble_functions() {
        let mut function_chunk = Chunk::default();
        function_chunk.add_constant(ValueType::Text(Box::new(String::from("Hello"))));
        function_chunk.add_op(Op::Const(0));
        function_chunk.add_op(Op::Return);
        let function = ValueType::Function(Box::new(Function::new(
            "greet".to_string(),
            function_chunk,
            0,
        )));

        let mut script_chunk = Chunk::default();
        script_chunk.add_constant(function);
        script_chunk.add_op(Op::Const(0));
        script_chunk.add_op(Op::Call(0));
        script_chunk.add_op(Op::Print);

        let out = test_disassemble(&script_chunk);
        let mut lines = out.lines();

        assert_eq!(lines.nth(8), Some("fn:greet:"));
        assert_eq!(lines.nth(1), Some("\t0000\ts:Hello"));
        assert_eq!(lines.nth(1), Some("\t0000\tCONST, 0"));
        assert_eq!(lines.next(), Some("\t0001\tRET"));
    }
}
