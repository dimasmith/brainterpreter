It is the reference of the instructions supported by the brainterpreter VM.

A full list of supported instructions is available in `src/vm/opcodes.rs`.

Supported operations can be split into a few groups based on their functions. 
The groups are:

- Placing constant values on stack;
- Arithmetic and logical operations;
- Jump to different instructions;
- Loading and storing global variables;
- Loading and storing local variables;
- Accessing and storing array elements;
- Calling functions;

# Placing constants on stack

Those operations place either immediate values or elements of constants array on the stack.

| Mnemonics | Parameters | Effect |
| :--- | :---- | :--- |
| `CONST_NIL` | None | Pushes `nil` on stack |
| `CONST_B <b>` | `b` immediate bolean value | Places boolean constant on stack |
| `CONST_F <f>` | `f` immediate numeric value | Places a numeric constant on stack |
| `CONST <n>` | `n` index of constant in the chunk constants | Copies chunk constant to the stack |

Placing values on stack makes them available for other manipulations like arithmetic or logical operations.

# Arithmetics and logic

Arithmetic and logic operations manipulates topmost values of the stack.
The result of the operation is pushed back on the stack.

Operations aren't applicable to all data types. 
E.g., division of strings does not make sense.
The virtual machine will stop with the error when the operation is not applicable to operands on the stack.

## Unary operations

Unary operations picks the last value from the stack, performs the operation and places the result on stack. 
It may seem as just replacing the value on the top of the stack.

| Mnemonics | Type | Effect |
| :--- | :--- | :--- |
| `NEG` | Boolean | Logical `not` operation |
| `NEG` | Number | Negates the value on top of the stack |

## Binary operations

Binary operations takes two operans from the stack, performs the operation and places the result back on stack. 

The first operand of the binary operation should be placed on top of the stack. 
The second operand is on the `top - 1` position.

E.g., for the operation `x - y` the stack layout should be like that.

:include-svg: img/binary-operation-stack-layout.svg {actualSize: true}

| Mnemonics | Type | Effect |
| :--- | :--- | :--- |
| `ADD` | Number, Number | Adds two numbers |
| `ADD` | String, String | Concatenates strings |
| `SUB` | Number, Number | Subtract second number from the first |
| `MUL` | Number, Number | Multiplies two numbers |
| `DIV` | Number, Number | Divides first number with second |
| `CMP` | Number, Number | Compares two numbers. Places `true` or `false` on stack |
| `CMP` | Boolean, Boolean | Compares two booleans |
| `CMP` | String, String | Compares two strings |
| `LE` | Number, Number | Pushes `true` if the first number is less or equal comparing to the second |
| `GE` | Number, Number | Pushes `true` if the first number is greater or equal comparing to the second |

# Jumping around

There are currently two jump calls allowing VM to change the execution flow. 
Each jump operation accepts the `offset` value. 
When applied, jump changes the instruction pointer by adding the offset to the current value.
Note that the offset can be negative.

| Mnemonics |  Effect |
| :--- | :--- |
| `JMP <offset>` | Unconditionally changes the instruction pointer by adding the offset | 
| `JZ <offset>` | Changes the instruction pointer only if the value on the top of the stack is `falze`. If not, the execution proceeds to next operation | 

# Global variables

Global variable instructions access global variables by the name.

Loading the variable places the value on stack.
Storing the variable replaces the value in globals map via the topmost value on the stack.

| Mnemonics | Parameters | Effect |
| :--- | :---- | :--- |
| `LD_G <name>` | name - the name of the variable | Copies the value of the global variable onto the stack |
| `ST_G <name>` | name - the name of the variable | Copies the value from the top of the stack to the globals map |

Warning: the mnemonics of globals instructions is about to change. The `name` parameter will refer to the index in the constant pool rather than the actual variable name.

# Local variables

Local variables are not resolved by the name. 
Instead of that the compiler places values on the fixed positions of the stack.
Thus instructions operating on local variables takes the offset from the *base* of the stack to address the variable.

| Mnemonics | Parameters | Effect |
| :--- | :---- | :--- |
| `LD_L <idx>` | idx - offset of the variable from the stack base | Copies stack value representing the local variable onto the stack top |
| `ST_L <idx>` | idx - offset of the variable from the stack base | Copies the value from the top of the stack to a stack position representing the local variable |

# Arrays

Array operations access elements of arrays and strings by index.

Array operations does not have parameters.
Instead they need all parameters to be correctly placed on the stack.

| Mnemonics | Stack Top | Top - 1 | Top - 2 | Effect |
| :--- |:--- | :--- | :--- | :--- |
| `ARR` | initial value | size | None | Allocates an array of specified size filled with initial value. Places the reference on the stack
| `LD_IDX` | array reference | index | none | Copies the value with specified index from array to the stack |
| `ST_IDX` | value | array reference | index | Replaces value in array with the value from the top of the stack |

Array access operation will fail on attempt to access values by index outside of the array.

# Function calls

| Mnemonics | Parameters | Effect |
| :-- | :-- | :-- |
| `CALL <arity>` | `arity` - number of function parameters | Calls the function. The function reference must be present at the `stack top - arity - 1` stack element. Call operation creates a call frame for the function and starts processing the function chunk.
| `RET` | None | Finishes the function. Removes all arguments from the stack. Places the return value (or `nil`) on the stack |

# Other instructions

| Mnemonics | Parameters | Effect |
| :-- | :-- | :-- |
| `POP` | None | Pop the value off the stack. Useful for cleaning up after finishing block statements or expression statements |
| `PRN` | None | Prints the value from the top of the stack to the linked output. Removes top value from the stack |
