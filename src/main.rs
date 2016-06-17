extern crate calc;
use std::io;
use std::io::Write;
use calc::parse;
use calc::interpret;

fn main() {
    print!("> ");
    io::stdout().flush().ok().unwrap();
    let mut s: String = "".to_string();
    match io::stdin().read_line(&mut s) {
        Ok(_) => {}
        Err(err) => { panic!(err); }
    }
    let ast = parse::parse(&s);
    println!("{:?}", ast);
    let result = interpret::f(&ast);
    println!("result = {:?}", result);
}
