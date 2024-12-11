// Reference: https://github.com/kevinmehall/rust-peg/blob/master/tests/test_arithmetic_ast.rs
/*
 * Notice: This code emits warnings (dead_code), which seems wrong. See https://github.com/rust-lang/rust/issues/27257
 */

use crate::ast::{FunDec, Op, Type, AST};

/*
 * Example: ([(1, +), (2, -)], 4) ==> (1 + 2) - 4
 * Note that va.0 is in reverse order.
 */
fn vecast_to_ast<T, F>(va: (Vec<(AST, T)>, AST), fold: F) -> AST
where
    F: Fn(T, AST, AST) -> AST,
    T: Copy,
{
    let (mut x, y) = va;
    if x.is_empty() {
        return y;
    }
    x.reverse();
    let mut ast = x[0].0.clone();
    for i in 0..x.len() {
        ast = fold(
            x[i].1,
            ast,
            if i == x.len() - 1 {
                y.clone()
            } else {
                x[i + 1].0.clone()
            },
        );
    }
    ast
}

peg::parser! {
    grammar arithmetic() for str {
        pub rule top_exp() -> (Vec<FunDec>, AST)
            = space()* f:fundec_space()* e:exp() space()* { (f, e) }
        rule fundec_space() -> FunDec
            = f:fundec() space()* { f }
        rule fundec() -> FunDec
            = "def" space()+ f:ident() space()*
            "(" space()* params:params() space()* ")" space()* ":" space()* ret_ty:type_() space()*
            "{" space()* e:exp() space()* "}" { {let mut t = params; t.reverse(); FunDec(f, t, ret_ty, e)} }
        rule params() -> Vec<(String, Type)>
            = x:ident() space()* ":" space()* ty:type_() space()* "," space()* params:params()
            { {let mut t = params; t.push((x, ty)); t} }
            / x:ident() space()* ":" space()* ty:type_() { vec![(x, ty)] }
        rule exp() -> AST
            = letex()
            / add_ast()
        rule letex() -> AST
            = "let" space()+ x:ident() space()* "=" space()* e1:exp() space()+ "in" space()+ e2:exp()
            { AST::LetEx(x, Box::new(e1), Box::new(e2)) }
        rule add_ast() -> AST
            = e:sum() { super::vecast_to_ast(e, |op, l, r| AST::OpNode(op, Box::new(l), Box::new(r))) }
        rule sum() -> (Vec<(AST, Op)>, AST)
            = l:mul_ast() space()* op:addop() space()* r:sum() { {let (mut x, y) = r; x.push((l, op)); (x, y)} }
            / e:mul_ast() { (Vec::new(), e) }
        rule addop() -> Op
            = "+" { Op::Add }
            / "-" { Op::Sub }
        rule mul_ast() -> AST
            = e:product() { super::vecast_to_ast(e, |op, l, r| AST::OpNode(op, Box::new(l), Box::new(r))) }
        rule product() -> (Vec<(AST, Op)>, AST)
            = l:atom() space()* op:mulop() space()* r:product() { {let (mut x, y) = r; x.push((l, op)); (x, y)} }
            / e:atom() { (Vec::new(), e) }
        rule mulop() -> Op
            = "*" { Op::Mul }
            / "/" { Op::Div }
        rule atom() -> AST
            = number()
            / str()
            / "if" space()* cond:exp() space()* "then" space()* e1:exp() space()*
                "else" space()* e2:exp() space()* "end"
                { AST::IfNode(Box::new(cond), Box::new(e1), Box::new(e2)) }
            / v:ident() space()* "(" space()* e:args() space()* ")"
                { {let mut t = e; t.reverse(); AST::FunApp(v, t)} }
                / v:ident() { AST::Var(v) }
            / "(" space()* v:exp() space()* ")" { v }
        rule number() -> AST
            = mstr:$(['0'..='9']+) { AST::Num(mstr.parse().unwrap()) }
        rule args() -> Vec<AST>
            = e:exp() space()* "," space()* a:args() { {let mut t = a; t.push(e); t} }
            / e:exp() { vec![e] }
        rule str() -> AST
            = "\"" s:str_internal() "\"" { AST::Str(s) }
        rule str_internal() -> String
            = mstr:$([^'\\' | '"']*) { mstr.to_string() }
        rule space() -> ()
            = " " / "\n" / "\r"
        rule ident() -> String
            = !keyword() mstr:$(['a'..='z' | 'A'..='Z']+) { mstr.to_string() }
        rule type_() -> Type
            = "int" { Type::Int }
            / "str" { Type::Str }
        rule keyword() -> () = "def" / "let" / "in" / "if" / "then" / "else"
    }
}

pub fn parse(s: &str) -> (Vec<FunDec>, AST) {
    match arithmetic::top_exp(s) {
        Ok(ast) => ast,
        Err(err) => {
            println!("{:?}", err);
            panic!("{err}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_test() {
        assert_eq!(
            parse("4 -2").1,
            AST::OpNode(Op::Sub, Box::new(AST::Num(4)), Box::new(AST::Num(2)))
        );
        assert_eq!(
            parse("let x = 4 in x + y * 2").1,
            AST::LetEx(
                "x".to_string(),
                Box::new(AST::Num(4)),
                Box::new(AST::OpNode(
                    Op::Add,
                    Box::new(AST::Var("x".to_string())),
                    Box::new(AST::OpNode(
                        Op::Mul,
                        Box::new(AST::Var("y".to_string())),
                        Box::new(AST::Num(2))
                    ))
                ))
            )
        );
    }
}
