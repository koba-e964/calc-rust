// Reference: https://github.com/kevinmehall/rust-peg/blob/master/tests/test_arithmetic_ast.rs
/*
 * Notice: This code emits warnings (dead_code), which seems wrong. See https://github.com/rust-lang/rust/issues/27257
 */


use ast::{AST, FunDec};


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

peg! arithmetic(include_str!("grammar.rustpeg"));


pub fn parse(s: &str) -> (Vec<FunDec>, AST) {
    match arithmetic::top_exp(s) {
        Ok(ast) => ast,
        Err(err) => { println!("{:?}", err); panic!(err) }
    }
}

#[cfg(test)]
mod tests {
    use parse::*;
    use ast::*;
    #[test]
    fn parse_test() {
        assert_eq!(parse("4 -2").1, AST::OpNode(Op::Sub, Box::new(AST::Num(4)), Box::new(AST::Num(2))));
        assert_eq!(parse("let x = 4 in x + y * 2").1,
            AST::LetEx("x".to_string(), Box::new(AST::Num(4)),
                       Box::new(AST::OpNode(Op::Add, Box::new(AST::Var("x".to_string())),
                                    Box::new(AST::OpNode(Op::Mul, Box::new(AST::Var("y".to_string())), Box::new(AST::Num(2)))))))
        );
    }
}
