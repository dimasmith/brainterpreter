use crate::value::ValueType;
use crate::vm::{VmRuntimeError, VmStack, STACK_SIZE};

impl VmStack {
    pub fn pop(&mut self) -> Result<ValueType, VmRuntimeError> {
        self.stack.pop().ok_or(VmRuntimeError::StackExhausted)
    }

    pub fn get(&self, offset: usize) -> Option<&ValueType> {
        self.stack.get(offset)
    }

    pub fn peek(&self, offset: usize) -> Option<&ValueType> {
        self.stack.get(self.stack.len() - offset - 1)
    }

    pub fn last(&self) -> Option<&ValueType> {
        self.stack.last()
    }

    pub fn len(&self) -> usize {
        self.stack.len()
    }

    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    pub fn push(&mut self, value: ValueType) {
        self.stack.push(value);
    }

    pub fn set(&mut self, offset: usize, value: ValueType) -> Result<(), VmRuntimeError> {
        if let Some(v) = self.stack.get_mut(offset) {
            *v = value;
            Ok(())
        } else {
            Err(VmRuntimeError::StackExhausted)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_value_by_offset() {
        let mut stack = VmStack::default();
        stack.push(ValueType::Number(1.0));
        stack.push(ValueType::Number(2.0));
        stack.set(0, ValueType::Number(3.0)).unwrap();
        stack.set(1, ValueType::Number(4.0)).unwrap();
        assert_eq!(stack.stack[0], ValueType::Number(3.0));
        assert_eq!(stack.stack[1], ValueType::Number(4.0));
    }
}

impl Default for VmStack {
    fn default() -> Self {
        let stack = Vec::with_capacity(STACK_SIZE);
        VmStack { stack }
    }
}
