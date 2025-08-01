//! Disassembler for the chunks of bytecode.
//!
//! It's a diagnostic tool to help find issues in compiled code.

use crate::value::ValueType;
use crate::vm::exec::Chunk;
use crate::vm::opcode::Op;
use std::io::{Error, Write};

/// Disassemble executable chunk into VM assembly.
pub fn disassemble(chunk: &Chunk, mut w: impl Write) -> Result<(), Error> {
    disassemble_function(chunk, "$main$", &mut w)
}

fn disassemble_function(chunk: &Chunk, name: &str, w: &mut impl Write) -> Result<(), Error> {
    let mut functions = vec![];
    writeln!(w, "fn:{name}:")?;
    writeln!(w, "constants:")?;
    for (pos, val) in chunk.constants().enumerate() {
        writeln!(w, "\t{pos:04x}\t{val}")?;
        if let ValueType::Function(function) = val {
            functions.push(function);
        }
    }
    writeln!(w, "code:")?;
    for (line, op) in chunk.ops().enumerate() {
        match op {
            Op::Jump(offset) | Op::JumpIfFalse(offset) => {
                let address = line.checked_add_signed(*offset as isize).unwrap();
                writeln!(w, "\t{line:04x}\t{op} # {address:04x}")?;
            }
            Op::StoreGlobal(idx) | Op::LoadGlobal(idx) => {
                let var_name = chunk.constant(*idx).unwrap().as_string();
                writeln!(w, "\t{line:04x}\t{op} # {var_name}")?;
            }
            o => writeln!(w, "\t{line:04x}\t{o}")?,
        }
    }
    writeln!(w)?;
    for function in functions.iter() {
        disassemble_function(&function.chunk(), function.name(), w)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value::{Function, ValueType};
    use crate::vm::opcode::Op;
    use std::io::BufWriter;
    use std::rc::Rc;

    fn test_disassemble(chunk: &Chunk) -> String {
        let mut w = BufWriter::new(vec![]);
        disassemble(chunk, &mut w).unwrap();
        let buf = w.into_inner().unwrap();
        String::from_utf8(buf).unwrap()
    }

    #[test]
    fn disassemble_single_instruction() {
        let chunk = Chunk::new([Op::Return], []);

        let out = test_disassemble(&chunk);
        let mut lines = out.lines();

        assert_eq!(lines.nth(3), Some("\t0000\tRET"));
    }

    #[test]
    fn disassemble_instructions_with_parameters() {
        let chunk = Chunk::new([Op::ConstFloat(3.42), Op::ConstBool(true)], []);

        let out = test_disassemble(&chunk);
        let mut lines = out.lines();

        assert_eq!(lines.nth(3), Some("\t0000\tCONST_F, 3.42"));
        assert_eq!(lines.next(), Some("\t0001\tCONST_B, true"));
    }

    #[test]
    fn disassemble_jump_instructions() {
        let chunk = Chunk::new(
            [
                Op::ConstFloat(5.0),
                Op::ConstFloat(1.0),
                Op::Add,
                Op::Jump(-2),
            ],
            [],
        );

        let out = test_disassemble(&chunk);
        let mut lines = out.lines();

        assert_eq!(lines.nth(6), Some("\t0003\tJMP, -2 # 0001"));
    }

    #[test]
    fn disassemble_string_constants() {
        let chunk = Chunk::new(
            [Op::Const(0)],
            [ValueType::Text(Box::new(String::from("Hello, World!")))],
        );

        let out = test_disassemble(&chunk);
        let mut lines = out.lines();

        assert_eq!(lines.nth(2), Some("\t0000\ts:Hello, World!"));
    }

    #[test]
    fn disassemble_functions() {
        let function_chunk = Chunk::new(
            [Op::Const(0), Op::Return],
            [ValueType::Text(Box::new(String::from("Hello")))],
        );
        let function = ValueType::Function(Box::new(Function::new(
            "greet".to_string(),
            Rc::new(function_chunk),
            0,
        )));

        let script_chunk = Chunk::new([Op::Const(0), Op::Call(0), Op::Print], [function]);

        let out = test_disassemble(&script_chunk);
        let mut lines = out.lines();

        assert_eq!(lines.nth(8), Some("fn:greet:"));
        assert_eq!(lines.nth(1), Some("\t0000\ts:Hello"));
        assert_eq!(lines.nth(1), Some("\t0000\tCONST, 0"));
        assert_eq!(lines.next(), Some("\t0001\tRET"));
    }
}
