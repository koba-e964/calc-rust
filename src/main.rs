// Reference: https://github.com/kevinmehall/rust-peg/blob/master/tests/test_arithmetic_ast.rs
#![feature(plugin)]
#![plugin(peg_syntax_ext)]

// AST
#[derive(PartialEq, Clone, Debug)]
pub enum AST {
    Num(i64),
    Str(String),
    Var(String),
    Sum(Box<AST>, Box<AST>),
    Prod(Box<AST>, Box<AST>),
    LetEx(String, Box<AST>, Box<AST>),
}

use arithmetic::exp;

peg! arithmetic(r#"
use AST;

#[pub]
exp -> AST
	= letex
        / sum
letex -> AST
        = "let" space+ x:var space* "=" space* e1:exp space+ "in" space+ e2:exp
           { AST::LetEx(x, Box::new(e1), Box::new(e2)) }
sum -> AST
	= l:product space* "+" space* r:sum { AST::Sum(Box::new(l), Box::new(r)) }
	/ product
product -> AST
	= l:atom space* "*" space* r:product { AST::Prod(Box::new(l), Box::new(r)) }
	/ atom
atom -> AST
	= number
        / str
        / v:var { AST::Var(v) }
	/ "(" space* v:exp space* ")" { v }
number -> AST
	= [0-9]+ { AST::Num(match_str.parse().unwrap()) }
str -> AST
        = "\"" "\"" { AST::Str("".to_string())}
space -> ()
        = " "
var -> String
        = [a-zA-Z]+ { match_str.to_string() }
"#);

fn main() {
    println!("{:?}", exp("let x = 4 in x + 2"));
}
