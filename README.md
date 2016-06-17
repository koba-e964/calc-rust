# CalcRust [![Build Status](https://travis-ci.org/koba-e964/calc-rust.svg?branch=master)](https://travis-ci.org/koba-e964/calc-rust)

This is a toy example of a calculator in Rust, using [`rust-peg`](https://github.com/kevinmehall/rust-peg).

## Grammar
Grammar is defined in PEG (Parsing Expression Grammar).
This calculator supports

- variable declaration (`let` expression)
- function definition (NOT implemented)

See `src/parse.rs` for more detail.

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
AddNode(Sub, MulNode(Mul, Num(3), Num(4)), Num(5))
result = VNum(7)
$ cargo run
     Running `target/debug/calc-rust`
> let x = 4 in x + x
LetEx("x", Num(4), AddNode(Add, Var("x"), Var("x")))
thread 'main' panicked at 'explicit panic', src/interpret.rs:30
note: Run with `RUST_BACKTRACE=1` for a backtrace.
error: Process didn't exit successfully: `target/debug/calc-rust` (exit code: 101)
```