use ast::{AST, FunDec, Value, Op};
use std::collections::HashMap;

fn f_sub(fundecs: &[FunDec], ast: &AST, env: &mut HashMap<String, Value>) -> Value {
    match *ast {
        AST::Num(i) => Value::VNum(i),
        AST::Str(ref str) => Value::VStr(str.clone()),
        AST::OpNode(Op::Add, ref e1, ref e2) =>
            match (f_sub(fundecs, e1, env), f_sub(fundecs, e2, env)) {
                (Value::VNum(i1), Value::VNum(i2)) => Value::VNum(i1 + i2),
                (Value::VStr(s1), Value::VStr(s2)) => { let mut s = s1.to_string(); s.push_str(&s2); Value::VStr(s)},
                _ => panic!("+ failed"),
            },
        AST::OpNode(Op::Sub, ref e1, ref e2) =>
            match (f_sub(fundecs, e1, env), f_sub(fundecs, e2, env)) {
                (Value::VNum(i1), Value::VNum(i2)) => Value::VNum(i1 - i2),
                _ => panic!("- failed"),
            },

        AST::OpNode(Op::Mul, ref e1, ref e2) =>
            match (f_sub(fundecs, e1, env), f_sub(fundecs, e2, env)) {
                (Value::VNum(i1), Value::VNum(i2)) => Value::VNum(i1 * i2),
                _ => panic!("* failed"),
            },
        AST::OpNode(Op::Div, ref e1, ref e2) =>
            match (f_sub(fundecs, e1, env), f_sub(fundecs, e2, env)) {
                (Value::VNum(i1), Value::VNum(i2)) => Value::VNum(i1 / i2),
                _ => panic!("/ failed"),
            },
        AST::IfNode(ref cond, ref e_true, ref e_false) => 
            match f_sub(fundecs, cond, env) {
                Value::VNum(0) => f_sub(fundecs, e_false, env),
                Value::VNum(_) => f_sub(fundecs, e_true, env),
                _ => panic!("Condition of if has to be an integer."),
            },
        AST::Var(ref x) => env.get(x).expect("variable not found").clone(),
        AST::LetEx(ref x, ref e1, ref e2) => {
            let v1 = f_sub(fundecs, e1, env);
            let old = env.insert(x.clone(), v1);
            let v2 = f_sub(fundecs, e2, env);
            env.remove(x).unwrap();
            if let Some(o) = old {
                env.insert(x.clone(), o);
            }
            v2
        }
        AST::FunApp(ref f, ref es) => {
            // evaluate arguments from left to right
            let n = es.len();
            let mut args = vec![Value::VNum(0); n];
            for i in 0 .. n {
                args[i] = f_sub(fundecs, &es[i], env);
            }
            let mut cp_env = env.clone();
            let fundec = fundecs.iter()
                .filter(|fundec| fundec.0 == *f).next()
                .expect("function not found");
            let m = fundec.1.len(); // #param
            if n != m {
                panic!("The number of parameters does not match the number of arguments.");
            }
            for i in 0 .. n {
                cp_env.insert(fundec.1[i].0.clone(), args[i].clone()); // TODO This second cloning is unnecessary. 
            }
            f_sub(fundecs, &fundec.3, &mut cp_env)
        },
    }
}

pub fn f(fundecs: &[FunDec], ast: &AST) -> Value {
    f_sub(fundecs, ast, &mut HashMap::new())
}

#[cfg(test)]
mod tests {
    use parse;
    use interpret;
    use ast::{AST, Op, Value};
    #[test]
    fn operations_test() {
        let ast1 = AST::OpNode(Op::Sub, Box::new(AST::Num(7)), Box::new(AST::Num(4)));
        assert_eq!(interpret::f(&Vec::new(), &ast1), Value::VNum(3));
        let ast2 = AST::OpNode(Op::Div, Box::new(AST::Num(20)), Box::new(AST::Num(4)));
        assert_eq!(interpret::f(&Vec::new(), &ast2), Value::VNum(5));
    }
    #[test]
    fn letex_test() {
        let ast1 = parse::parse("let x = 4 in x + x");
        assert_eq!(interpret::f(&ast1.0, &ast1.1), Value::VNum(8));
        let ast2 = parse::parse("let x = 4 in let x = 3 in x + x");
        assert_eq!(interpret::f(&ast2.0, &ast2.1), Value::VNum(6));
        let ast3 = parse::parse("let x = 4 in (let x = 3 in x) + x");
        assert_eq!(interpret::f(&ast3.0, &ast3.1), Value::VNum(7));
    }
}
