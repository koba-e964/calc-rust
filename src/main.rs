// Reference: https://github.com/kevinmehall/rust-peg/blob/master/tests/test_arithmetic_ast.rs
#![feature(plugin)]
#![plugin(peg_syntax_ext)]

// AST
#[derive(PartialEq, Clone, Debug)]
pub enum AST {
    Num(i64),
    Str(String),
    Var(String),
    AddNode(AddOp, Box<AST>, Box<AST>),
    MulNode(MulOp, Box<AST>, Box<AST>),
    LetEx(String, Box<AST>, Box<AST>),
}
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum AddOp {
    Add, Sub,
}
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum MulOp {
    Mul, Div,
}


use arithmetic::exp;

/*
 * Example: ([(1, +), (2, -)], 4) ==> (1 + 2) - 4
 */
fn vecast_to_ast<T, F>(va: (Vec<(AST, T)>, AST), fold: F) -> AST
where F: Fn(T, AST, AST) -> AST, T: Copy {
    let (x, y) = va;
    if x.len() == 0 {
        return y;
    }
    let mut ast = x[0].0.clone();
    for i in 0 .. x.len(){
        ast = fold(x[i].1, ast, if i == x.len() - 1 { y.clone() } else { x[i + 1].0.clone() });
    }
    ast
}

peg! arithmetic(r#"
use AST;
use AddOp;
use MulOp;

#[pub]
exp -> AST
	= letex
        / add_ast
letex -> AST
        = "let" space+ x:var space* "=" space* e1:exp space+ "in" space+ e2:exp
           { AST::LetEx(x, Box::new(e1), Box::new(e2)) }
add_ast -> AST
        = e:sum { super::vecast_to_ast(e, |op, l, r| AST::AddNode(op, Box::new(l), Box::new(r))) }
sum -> (Vec<(AST, AddOp)>, AST)
	= l:mul_ast space* op:addop space* r:sum { {let (mut x, y) = r; x.insert(0, (l, op)); (x, y)} } // TODO very inefficient insert.
	/ e:mul_ast { (Vec::new(), e) }
addop -> AddOp
        = "+" { AddOp::Add }
        / "-" { AddOp::Sub }
mul_ast -> AST
        = e:product { super::vecast_to_ast(e, |op, l, r| AST::MulNode(op, Box::new(l), Box::new(r))) }
product -> (Vec<(AST, MulOp)>, AST)
	= l:atom space* op:mulop space* r:product { {let (mut x, y) = r; x.insert(0, (l, op)); (x, y)} } // TODO very inefficient insert.
	/ e:atom { (Vec::new(), e) }
mulop -> MulOp
        = "*" { MulOp::Mul }
        / "/" { MulOp::Div }
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
    println!("{:?}", exp("let x = 4 in x - y + 2 - 3"));
}
