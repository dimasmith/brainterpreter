Warning: Documentation is under active development (fancy word for incomplete). 

# Brainterpreter

Brainterpreter is an implementation of a toy programming language called `Bauble`.

What's the implementation of the programming language? 
Well, that might be different for real languages. 
But Bauble has the following components:

- Parser - parses the source code and produces the AST;
- Compiler - compiles the AST into chunks of the bytecode;
- VM - virtual machine running compiled chunks;

# Bauble

Bauble is a toy programming language for fun and education.
Look - we even have a nice logo!

:include-image: img/bauble-logo.png {title: "Bauble Logo", align: "left", scale: 0.2}

The language is very simple. 
The syntax is c-like. 
The list of features is very modest (e.g., it does not support for loops yet) 
but it has enough to be Turing-complete.
For the conference I implemented the Brainfuck interpreter using Bauble.

You can get the gist of language with this example.

```javascript
let memorySize = 256 * 256;
let memory = [0; memorySize];  // Fixed-size array initialized with 0 values
let memPointer = 0;

fun increment() {
    let value = memory[memoryPointer];
    if (value < 254) {
        value = value + 1;
    } else {
        value = 0;
    }
    memory[memoryPointer] = value;
    
}

increment();

print memory[memPointer];
```

Complete syntax reference can be found in the [syntax section](language/syntax).

Do not: use Bauble in production. It's a toy. It has its name for a reason. 

# History 
The tool suite is created for a TechIn conference talk called `Brainterpreter`.

The initial idea of the talk was to create a [Brainfuck](https://esolangs.org/wiki/Brainfuck) interpreter.
But it was hard to represent various aspects of parsing using Brainfuck.
So I decided to create a simple procedural programming language and implement the Brainfuck interpreter using that language.

That's how the Brainterpreter was created.

# Inspiration

[Crafting Interpreters](https://craftinginterpreters.com/) by Robert Nystrom is an incredible source of inspiration for creating programming languages. Bauble was born from this inspiration. In "Crafting Interpreters", Robert builds a `Lox` programming language. Lox is similar to Bauble but much more robust and complete. The book provides two implementations of Lox, one in Java and one in C, each with its unique approach. The Java implementation constructs an AST and executes it using JVM, while the C implementation requires building every aspect of the language from the lexer to the virtual machine. 

Brainterpreter is based on the C implementation of Lox. It has a different parser implementation that builds an AST, while Lox compiles directly from the parser in its C implementation.

If you're interested in creating programming languages but don't know where to start, "Crafting Interpreters" is a must-read.