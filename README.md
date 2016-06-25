# CalcRust [![Build Status](https://travis-ci.org/koba-e964/calc-rust.svg?branch=master)](https://travis-ci.org/koba-e964/calc-rust)


This is a toy example of a calculator in Rust, using [`rust-peg`](https://github.com/kevinmehall/rust-peg).

## Grammar
The grammar is defined in PEG (Parsing Expression Grammar).
This calculator supports

- variable declaration (`let` expression)
- conditional expression (`if`-`else` expression)
- function definition

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
    -t, --typing   Check types
```
and described in `src/main.rs`. The output will be like:
```
$ cargo run -- -t
     Running `target/debug/calc-rust -t`
> let x = 4 in x + 5
fundecs: []
LetEx("x", Num(4), OpNode(Add, Var("x"), Num(5)))
typing: ([], LetEx("x", Int, Num(4), OpNode(Add, Int, Var("x", Int), Num(5))))
result = VNum(9)
$ cargo run -- -t sample/add.txt
     Running `target/debug/calc-rust -t sample/add.txt`
fundecs: []
OpNode(Add, OpNode(Add, Num(2), Num(3)), Num(4))
typing: ([], OpNode(Add, Int, OpNode(Add, Int, Num(2), Num(3)), Num(4)))
result = VNum(9)
$ cargo run -- -t sample/fundec.txt
     Running `target/debug/calc-rust -t sample/fundec.txt`
fundecs: [("id", [("x", Int)], Int, Var("x")), ("subtract", [("x", Int), ("y", Int)], Int, OpNode(Sub, Var("x"), Var("y")))]
FunApp("id", [FunApp("subtract", [Num(5), Num(7)])])
typing: ([("id", [("x", Int)], Int, Var("x", Int)), ("subtract", [("x", Int), ("y", Int)], Int, OpNode(Sub, Int, Var("x", Int), Var("y", Int)))], FunApp("id", [Int], Int, [FunApp("subtract", [Int, Int], Int, [Num(5), Num(7)])]))
result = VNum(-2)
$ cargo run -- -t sample/fib.txt
     Running `target/debug/calc-rust -t sample/fib.txt`
fundecs: [("fib", [("x", Int)], Int, IfNode(OpNode(Mul, Var("x"), OpNode(Sub, Var("x"), Num(1))), OpNode(Add, FunApp("fib", [OpNode(Sub, Var("x"), Num(1))]), FunApp("fib", [OpNode(Sub, Var("x"), Num(2))])), Var("x")))]
FunApp("fib", [Num(10)])
typing: ([("fib", [("x", Int)], Int, IfNode(OpNode(Mul, Int, Var("x", Int), OpNode(Sub, Int, Var("x", Int), Num(1))), Int, OpNode(Add, Int, FunApp("fib", [Int], Int, [OpNode(Sub, Int, Var("x", Int), Num(1))]), FunApp("fib", [Int], Int, [OpNode(Sub, Int, Var("x", Int), Num(2))])), Var("x", Int)))], FunApp("fib", [Int], Int, [Num(10)]))
result = VNum(55)
```