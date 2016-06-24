#![feature(plugin)]
#![plugin(docopt_macros)]

extern crate calc;
extern crate rustc_serialize;
extern crate docopt;

use std::io;
use std::io::{Read,Write};
use std::fs::File;
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
    print!("> ");
    io::stdout().flush().ok().unwrap();
    let mut s: String = "".to_string();
    if args.arg_INPUT == "".to_string() { // Reads from stdin
        match io::stdin().read_line(&mut s) {
            Ok(_) => {}
            Err(err) => { panic!(err); }
        }
    } else { // Reads from file
        let mut fp = File::open(args.arg_INPUT)
            .unwrap_or_else(|e| panic!(e));
        fp.read_to_string(&mut s);
    }
    let (fundecs, ast) = parse::parse(&s);
    println!("fundecs: {:?}", fundecs);
    println!("{:?}", ast);
    println!("typing: {:?}", typing::f(&ast));
    println!("result = {:?}", interpret::f(&ast));
}
