extern crate calc;
use std::io;
use std::io::Write;
use calc::parse;

fn main() {
    print!("> ");
    io::stdout().flush();
    let mut s: String = "".to_string();
    match io::stdin().read_line(&mut s) {
        Ok(_) => {}
        Err(err) => { panic!(err); }
    }
    let ast = parse::parse(&s);
    println!("{:?}", ast);
}
