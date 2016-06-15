// Reference: https://github.com/kevinmehall/rust-peg/blob/master/tests/test_arithmetic_ast.rs
#![feature(plugin)]
#![plugin(peg_syntax_ext)]

// AST
#[derive(PartialEq, Clone, Debug)]
pub enum AST {
    Number(i64),
    Sum(Box<AST>, Box<AST>),
    Prod(Box<AST>, Box<AST>),
}

use arithmetic::expression;

peg! arithmetic(r#"
use AST;
use AST::Sum;
use AST::Prod;
use AST::Number;

#[pub]
expression -> AST
	= sum
sum -> AST
	= l:product "+" r:product { Sum(Box::new(l), Box::new(r)) }
	/ product
product -> AST
	= l:atom "*" r:atom { Prod(Box::new(l), Box::new(r)) }
	/ atom
atom -> AST
	= number
	/ "(" v:sum ")" { v }
number -> AST
	= [0-9]+ { Number(match_str.parse().unwrap()) }
"#);

fn main() {
    println!("{:?}", expression("1+2*3"));
}
