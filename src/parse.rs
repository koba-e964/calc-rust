// Reference: https://github.com/kevinmehall/rust-peg/blob/master/tests/test_arithmetic_ast.rs
/*
 * Notice: This code emits warnings (dead_code), which seems wrong. See https://github.com/rust-lang/rust/issues/27257
 */


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



/*
 * Example: ([(1, +), (2, -)], 4) ==> (1 + 2) - 4
 * Note that va.0 is in reverse order.
 */
fn vecast_to_ast<T, F>(va: (Vec<(AST, T)>, AST), fold: F) -> AST
where F: Fn(T, AST, AST) -> AST, T: Copy {
    let (mut x, y) = va;
    if x.len() == 0 {
        return y;
    }
    x.reverse();
    let mut ast = x[0].0.clone();
    for i in 0 .. x.len(){
        ast = fold(x[i].1, ast, if i == x.len() - 1 { y.clone() } else { x[i + 1].0.clone() });
    }
    ast
}

peg! arithmetic(r#"
use parse::*;

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
	= l:mul_ast space* op:addop space* r:sum { {let (mut x, y) = r; x.push((l, op)); (x, y)} }
	/ e:mul_ast { (Vec::new(), e) }
addop -> AddOp
        = "+" { AddOp::Add }
        / "-" { AddOp::Sub }
mul_ast -> AST
        = e:product { super::vecast_to_ast(e, |op, l, r| AST::MulNode(op, Box::new(l), Box::new(r))) }
product -> (Vec<(AST, MulOp)>, AST)
	= l:atom space* op:mulop space* r:product { {let (mut x, y) = r; x.push((l, op)); (x, y)} }
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

#[cfg(test)]
mod tests {
    use parse::*;
    use parse::arithmetic::*;
    #[test]
    fn parse_test() {
        assert_eq!(exp("4 -2"), Ok(AST::AddNode(AddOp::Sub, Box::new(AST::Num(4)), Box::new(AST::Num(2)))));
        assert_eq!(exp("let x = 4 in x + y * 2"), Ok(
            AST::LetEx("x".to_string(), Box::new(AST::Num(4)),
                       Box::new(AST::AddNode(AddOp::Add, Box::new(AST::Var("x".to_string())),
                                    Box::new(AST::MulNode(MulOp::Mul, Box::new(AST::Var("y".to_string())), Box::new(AST::Num(2)))))))
        ));
    }
}
