The Brainterpreter is a set of components and abstractions implementing the Bauble language.

The diagram below shows the path from the source code to execution.

:include-svg: img/component.svg {actualSize: true, scale: 0.75}

The brainterpreter is a library providing those components. 
You may use them as is, or mix and match in different combination.

E.g., you can use the parser to produce AST, but to replace the compiler with more effective.

You may replace the virtual machine and compile to JVM class file instead.
Or you may parse different language into the AST and use brainpreter VM to run your code.

All the components are implemented in Rust programming language.
The default build of the brainterpreter package contains a library crate with main components.
You can use or replace components in your own programs.

# Parser

A parser is responsible for transforming the source code into the Abstract Syntax Tree (AST).

The implementation is based on the [Pratt Parser](https://en.wikipedia.org/wiki/Operator-precedence_parser) design.
The most helpful article for implementing the parser was the [Simple but powerful Pratt Parsing](https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html).

The parser itself consists of two components - the lexer and the parser itself.

Lexer converts a source file into a stream of tokens. 
Working with prepared tokens makes parser implementaton much simpler.

The parser itself is roughly divided into two parts:

- Expression parser;
- Statement parser;

Expression parser is responsible for processing parts of the program which produces values. 
Statements parser works with actionable program instructions.

# Abstract Synax Tree (AST)

The AST is a representation of program syntax which can be optimised and compiled to bytecode or other executable format.

Expressions and statements of the language has their representation in the AST.

Essentially, the whole bauble program is the list of statements.
Some examples of statements nodes are:

- While statement;
- If statement;
- Function definition statement;
- Block statement;

Anything that produces a data is represented via expressions in AST.
Some examples are:

- Literals: numbers, strings, arrays;
- Arithmetic and logical operations;
- Function calls;
- Variable assignments;

# Compiler

Compiler accepts AST and produces a chunk of a bytecode for a brainterpreter virtual machine.

The virtual machine is stack-based, so the compiler is pretty simple.

Compiler transforms the hierarchical AST into the plain list of VM instructions.
It calculates stack places of local variables and jump offsets for control flow operations.

# Chunk

Chunk is a list of instructions for virtual machine to execute.
Chunk also contains the list of constant values to simplify the instruction sets.

Warning: the chunk does not have a serializable representation yet. 
This is one of the future tasks.

:include-svg: img/chunk.svg {actualSize: true}

Constants list keeps values that are not included into the instructions as immediate values.
Some values that are recorded as constans are:

- Strings;
- Functions;
- Variable names;

Numbers are usually passed as immediate values. 
E.g., `CONST_N 2` instruction loads the number `2` onto the stack.
But the `CONST 2` instruction places the constant with index `2` onto the stack.

# Virtual Machine (VM)

The brainpreter virtual machine is a simple stack-based virtual machine with a small instruction set.

:include-svg: img/virtual-machine.svg {actualSize: true}

The VM consists of a few blocks.

## Value Stack

The Value Stack keeps working values of the program in a stack structure. 
The VM can access stack elements relatively to the stack top and by the offset from the stack base.

## Global Variables Map

Global variables map contains values of global variables mapped by the variable names.
The locals are instead stored on the stack and are never referenced by the name in the VM.

## Call Stack

Call stack contains call frames. 
Call frames simplify working with function calls.
The call frame on top defines parameters for currently running function.
It specifies the base of the stack for the function and keeps a reference to a chunk of function code.

## Instructions

VM keeps a pointer to the instruction that is about to be executed. 
On each step the VM loads the next instruction from the chunk and executes it.
