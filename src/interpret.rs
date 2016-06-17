use ast::{AST, Value, AddOp, MulOp};

pub fn f(ast: &AST) -> Value {
    match *ast {
        AST::Num(i) => Value::VNum(i),
        AST::Str(ref str) => Value::VStr(str.clone()),
        AST::AddNode(AddOp::Add, ref e1, ref e2) =>
            match (f(e1), f(e2)) {
                (Value::VNum(i1), Value::VNum(i2)) => Value::VNum(i1 + i2),
                (Value::VStr(s1), Value::VStr(s2)) => { let mut s = s1.to_string(); s.push_str(&s2); Value::VStr(s)},
                _ => panic!("+ failed"),
            },
        AST::AddNode(AddOp::Sub, ref e1, ref e2) =>
            match (f(e1), f(e2)) {
                (Value::VNum(i1), Value::VNum(i2)) => Value::VNum(i1 - i2),
                _ => panic!("- failed"),
            },

        AST::MulNode(MulOp::Mul, ref e1, ref e2) =>
            match (f(e1), f(e2)) {
                (Value::VNum(i1), Value::VNum(i2)) => Value::VNum(i1 * i2),
                _ => panic!("* failed"),
            },
        AST::MulNode(MulOp::Div, ref e1, ref e2) =>
            match (f(e1), f(e2)) {
                (Value::VNum(i1), Value::VNum(i2)) => Value::VNum(i1 / i2),
                _ => panic!("/ failed"),
            },
        AST::Var(_) => panic!(),
        AST::LetEx(_, _, _) => panic!(),
    }
}


#[cfg(test)]
mod tests {
    use parse;
    use interpret;
    use ast::{AST, AddOp, MulOp, Value};
    #[test]
    fn operations_test() {
        let ast1 = AST::AddNode(AddOp::Sub, Box::new(AST::Num(7)), Box::new(AST::Num(4)));
        assert_eq!(interpret::f(&ast1), Value::VNum(3));
        let ast2 = AST::MulNode(MulOp::Div, Box::new(AST::Num(20)), Box::new(AST::Num(4)));
        assert_eq!(interpret::f(&ast2), Value::VNum(5));
    }
    #[test]
    fn letex_test() {
        let ast1 = parse::parse("let x = 4 in x + x");
        assert_eq!(interpret::f(&ast1), Value::VNum(8));
        let ast2 = parse::parse("let x = 4 in let x = 3 in x + x");
        assert_eq!(interpret::f(&ast2), Value::VNum(6));
    }
}
