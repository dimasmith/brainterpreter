:include-svg: img/virtual-machine-architecture.svg {actualSize: true}

Main components of the virtual machine are:

- Value stack - keeps working values of the program;
- Call stack - keeps function calls and execution information;
- Globals - stores global variables;
- IO - references the IO channels to perform IO operations;
- Trace - pluggable VM execution tracing;

# Running programs

The virtual machine accepts a Chunk of bytecode and begins the execution.

Before running the program the call and value stack is empty, as well as the globals map. 

When execution starts, the VM creates a main Call Frame and places in on top of the Call Stack.
The call frame stores the pointer to the value stack base (always 0 when virtual machines just starts). 
It also keeps reference to a currently active chunk and the chunk instruction pointer.

The VM loads the instruction pointed via the IP of the active call frame and executes it.

:include-plantuml: img/boot-vm.puml

# Supported data types

The VM supports a modest set of data types. 

- Nil;
- Boolean;
- Number;
- Text (aka String);
- Function;
- Native function;
- Array;

The VM don't do any implicit conversions of the data types. 
Currently, the support of type conversion in Bauble is limited as well.
There are `to_char` and `to_string` native functions to partially mitigate the issue.

Any value can be placed onto the value stack and processed there.

# Error processing

As of now the virtual machine just fails on any error.
There is no way to recover from the issue.
This will change in future.