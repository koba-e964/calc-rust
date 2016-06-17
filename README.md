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

To build the interpreter, run
```
cargo run
```
in the root directory of this project. The output will be like:
```
$ cargo run
     Running `target/debug/calc-rust`
> let x = 4 in 5 * x - 3
LetEx("x", Num(4), AddNode(Sub, MulNode(Mul, Num(5), Var("x")), Num(3)))
```