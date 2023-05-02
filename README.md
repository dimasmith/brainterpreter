# Brainterpreter

[![Build](https://github.com/dimasmith/brainterpreter/actions/workflows/rust.yml/badge.svg)](https://github.com/dimasmith/brainterpreter/actions/workflows/rust.yml)
[![Coverage](https://codecov.io/gh/dimasmith/brainterpreter/branch/main/graph/badge.svg?token=ZCTAGTAWRJ)](https://codecov.io/gh/dimasmith/brainterpreter)
[![Documentation](https://github.com/dimasmith/brainterpreter/actions/workflows/znai-pages-deploy.yml/badge.svg)](https://github.com/dimasmith/brainterpreter/actions/workflows/znai-pages-deploy.yml)

An interpreter for a Bauble programming language.
Created for TechIn talks in [Levi9](https://www.levi9.com/).

NOTE: it is a sandbox of the implementation.
I plan to deliver a new repository with the same functionality but better suited for education.

An interpreter is created as an educational experiment.
The main goal is to run the interpreter of [Brainfuck](https://esolangs.org/wiki/Brainfuck) written in this toy language. 
You can check a "hello, world" in `examples` section.

## What's inside?

The repository containts parser, compiler, and virtual machine for a Bauble programming language.
The language has C-like syntax. Supported features:

- arithmetics;
- strings;
- boolean values;
- if statement;
- while loop;
- arrays;

## What's Bauble?

Bauble is a toy programming language created specifically for a tech talk.

## Documentation

Please check the [project pages](https://dimasmith.github.io/brainterpreter/) for some docs.

Documenting is still very much in progress.

## Running interpreter

By default, the project builds as a library with all parts to run the interpreter.
If you don't want to build your own binary, build brainterpreter with the `cli` feature enabled.

```shell
cargo build --features cli
```

The `cli` feature provides a `bauble` binary. 
`bauble` runs the interpreter for a given source file.

If you want to trace the virtual machine execution for your program, set logging level to `debug`:

```shell
RUST_LOG=debug bauble examples/hello_world.bbl run
```

After the build the binary can be found in `target/debug/bauble` (or `target/release/bauble` if you build with `--release` flag).

You can also run the interpreter with the `cargo run` command:

```shell
cargo run --features cli -- examples/hello_world.bbl
```
