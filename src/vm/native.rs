use crate::value::{NativeFunction, ValueType};
use crate::vm::{Vm, VmRuntimeError};

pub fn std_lib() -> Vec<NativeFunction> {
    vec![
        NativeFunction::new("len", 1, len),
        NativeFunction::new("as_char", 1, as_char),
        NativeFunction::new("as_string", 1, as_string),
    ]
}

fn len(vm: &mut Vm) -> Result<(), VmRuntimeError> {
    let value = vm.pop()?;
    vm.pop()?;
    let len = match value {
        ValueType::Text(text) => text.len(),
        ValueType::Array(array) => array.len(),
        _ => return Err(VmRuntimeError::TypeMismatch),
    };
    vm.push(ValueType::Number(len as f64));
    Ok(())
}

fn as_char(vm: &mut Vm) -> Result<(), VmRuntimeError> {
    let value = vm.pop()?;
    vm.pop()?;
    match &value {
        ValueType::Number(n) => {
            let c = *n as u8 as char;
            vm.push(ValueType::Text(Box::new(c.to_string())));
            Ok(())
        }
        _ => Err(VmRuntimeError::TypeMismatch),
    }
}

fn as_string(vm: &mut Vm) -> Result<(), VmRuntimeError> {
    let value = vm.pop()?;
    vm.pop()?;
    let string = value.as_string();
    vm.push(ValueType::Text(Box::new(string)));
    Ok(())
}
