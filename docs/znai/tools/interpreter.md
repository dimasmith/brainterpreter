The Brainterpreter includes the bauble interpreter binary, so you can run your source files directly.

# Building interpreter binary

The source of interpreter cli is the `src/bin/bauble.rs`.

The normal `cargo build` does not build the cli by default.
It only builds the `brainterpreter` library.
The interpreter cli is a part of `cli` build feature.

Use this command to build the interpreter cli.

```shell
cargo build --features="cli"
```

## Installing interpreter binary locally

You may want to install the binary locally for convenience.
Cargo can help you to do so using the command.

```shell
cargo install --features="cli" --path .
```

# Running bauble files

Create your source and save it with the `bbl` extension.
E.g., you have a `hello.bbl` file with the source:

```javascript
print "Hello, World!";
```

Run it with the command.

```shell
bauble hello.bbl
```

# Viewing virtual machine trace

The virtual machine provides a verbose diagnostic output while running the program.
It produces the output in the `trace` logging level.

Enable the trace output by using the `--trace` flag of the interpreter cli.

```shell
bauble --trace hello.bbl
```

Alternatively, you can enable the trace output by setting the `RUST_LOG` environment variable to `trace` level.

The example for nix systems:

```shell
RUST_LOG="trace" bauble hello.bbl
```

The diagnostic output of the VM looks includes instructions window and the stack.

```shell {commentsType: "inline"}
[DEBUG brainterpreter::log] ================  # start of instruction window
[DEBUG brainterpreter::log] = instructions
[DEBUG brainterpreter::log] 3:	ST_L 0        # a few previous instructions for context
[DEBUG brainterpreter::log] 4:	CONST_F, 1
[DEBUG brainterpreter::log] 5:>	LD_L 0        # current instruction
[DEBUG brainterpreter::log] ----------------
[DEBUG brainterpreter::log] = stack after     # state of the stack after the instruction execution
[DEBUG brainterpreter::log] 0:	fn:$main$
[DEBUG brainterpreter::log] 1:	s:-
[DEBUG brainterpreter::log] 2:	&[]
[DEBUG brainterpreter::log] 3:	f:0
[DEBUG brainterpreter::log] 4:	f:0
```

