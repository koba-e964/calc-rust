#![feature(plugin)]
#![plugin(docopt_macros)]

extern crate calc;
extern crate rustc_serialize;
extern crate docopt;

use std::io;
use std::io::Write;
use calc::parse;
use calc::interpret;
use calc::typing;

docopt!(Args, "
Usage: calc-rust [options] [INPUT]

Options:
    -v, --verbose  Verbose mode
");

fn main() {
    let args: Args = Args::docopt()
        .decode()
        .unwrap_or_else(|e| e.exit());
    if args.flag_verbose {
        println!("verbose mode");
    }
    println!("input file: {}", args.arg_INPUT);
    print!("> ");
    io::stdout().flush().ok().unwrap();
    let mut s: String = "".to_string();
    match io::stdin().read_line(&mut s) {
        Ok(_) => {}
        Err(err) => { panic!(err); }
    }
    let ast = parse::parse(&s);
    println!("{:?}", ast);
    println!("typing: {:?}", typing::f(&ast));
    println!("result = {:?}", interpret::f(&ast));
}
