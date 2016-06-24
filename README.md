# CalcRust [![Build Status](https://travis-ci.org/koba-e964/calc-rust.svg?branch=master)](https://travis-ci.org/koba-e964/calc-rust)


This is a toy example of a calculator in Rust, using [`rust-peg`](https://github.com/kevinmehall/rust-peg).

## Grammar
Grammar is defined in PEG (Parsing Expression Grammar).
This calculator supports

- variable declaration (`let` expression)
- function definition (NOT implemented)

See `src/grammar.rustpeg` for the grammar of this language.

## How to run
This software works only on nightly Rust, because of unstable features.

To run the interpreter, run
```
cargo run
```
in the root directory of this project.

The usage is
```
Usage: calc-rust [options] [INPUT]

Options:
    -v, --verbose  Verbose mode
```
and described in `src/main.rs`. The output will be like:
```
$ cargo run
     Running `target/debug/calc-rust`
> 3 * 4 - 5
fundecs: []
OpNode(Sub, OpNode(Mul, Num(3), Num(4)), Num(5))
result = VNum(7)
$ cargo run
     Running `target/debug/calc-rust`
> let x = 4 in 3 * x - 5
fundecs: []
LetEx("x", Num(4), OpNode(Sub, OpNode(Mul, Num(3), Var("x")), Num(5)))
result = VNum(7)
$ cargo run
     Running `target/debug/calc-rust`
> let x = 4 in x + "33"
fundecs: []
LetEx("x", Num(4), OpNode(Add, Var("x"), Str("33")))
thread 'main' panicked at '+ failed', src/interpret.rs:12
note: Run with `RUST_BACKTRACE=1` for a backtrace.
error: Process didn't exit successfully: `target/debug/calc-rust` (exit code: 101)
$ cargo run -- sample/add.txt
     Running `target/debug/calc-rust sample/add.txt`
fundecs: []
OpNode(Add, OpNode(Add, Num(2), Num(3)), Num(4))
result = VNum(9)
$ cargo run -- sample/fundec.txt
     Running `target/debug/calc-rust sample/fundec.txt`
fundecs: [("id", [("x", Int)], Var("x")), ("subtract", [("x", Int), ("y", Int)], OpNode(Sub, Var("x"), Var("y")))]
FunApp("id", [FunApp("subtract", [Num(5), Num(7)])])
result = VNum(-2)
```