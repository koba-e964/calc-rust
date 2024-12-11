use calc::interpret;
use calc::parse;
use calc::typing;
use clap::{Arg, Command};
use std::fs::File;
use std::io;
use std::io::{Read, Write};

struct Args {
    flag_verbose: bool,
    flag_typing: bool,
    arg_input: String,
}

fn get_args() -> Args {
    let matches = Command::new("calc-rust")
        .version("0.1.0")
        .author("koba-e964 <3303362+koba-e964@users.noreply.github.com>")
        .about("A calculator written in Rust")
        .arg(
            Arg::new("INPUT")
                .help("Input file")
                .required(false)
                .index(1)
                .value_name("INPUT"),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Verbose mode"),
        )
        .arg(
            Arg::new("typing")
                .short('t')
                .long("typing")
                .help("Check types"),
        )
        .get_matches();

    Args {
        flag_verbose: matches.contains_id("verbose"),
        flag_typing: matches.contains_id("typing"),
        arg_input: matches
            .get_one::<String>("INPUT")
            .unwrap_or(&"".to_string())
            .to_string(),
    }
}

fn main() {
    let args: Args = get_args();
    if args.flag_verbose {
        println!("verbose mode");
    }
    let mut s: String = "".to_string();
    if args.arg_input.is_empty() {
        // Reads from stdin
        print!("> ");
        io::stdout().flush().ok().unwrap();
        match io::stdin().read_line(&mut s) {
            Ok(_) => {}
            Err(err) => {
                panic!("{err}");
            }
        }
    } else {
        // Reads from file
        let mut fp = File::open(args.arg_input).unwrap_or_else(|e| panic!("{e}"));
        fp.read_to_string(&mut s).unwrap_or_else(|e| panic!("{e}"));
    }
    let (fundecs, ast) = parse::parse(&s);
    println!("fundecs: {:?}", fundecs);
    println!("{:?}", ast);
    if args.flag_typing {
        println!("typing: {:?}", typing::f(&fundecs, &ast));
    }
    println!("result = {:?}", interpret::f(&fundecs, &ast));
}
