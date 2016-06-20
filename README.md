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
in the root directory of this project. The output will be like:
```
$ cargo run
     Running `target/debug/calc-rust`
> 3 * 4 - 5
OpNode(Sub, OpNode(Mul, Num(3), Num(4)), Num(5))
typing: OpNode(Sub, Int, OpNode(Mul, Int, Num(3), Num(4)), Num(5))
result = VNum(7)
$ cargo run
     Running `target/debug/calc-rust`
> let x = 4 in 3 * x - 5
LetEx("x", Num(4), OpNode(Sub, OpNode(Mul, Num(3), Var("x")), Num(5)))
typing: LetEx("x", Int, Num(4), OpNode(Sub, Int, OpNode(Mul, Int, Num(3), Var("x", Int)), Num(5)))
result = VNum(7)
$ cargo run
     Running `target/debug/calc-rust`
> let x = 4 in x + "33"
LetEx("x", Num(4), OpNode(Add, Var("x"), Str("33")))
thread 'main' panicked at 'typing of Add failed', src/typing.rs:20
note: Run with `RUST_BACKTRACE=1` for a backtrace.
error: Process didn't exit successfully: `target/debug/calc-rust` (exit code: 101)
```