use ast::{AST, Type, FunDec, TypedAST, TypedFunDec, Op, ty_of_ast};
use std::collections::HashMap;

fn expect_type(fundecs: &[FunDec], ast: &AST, env: &mut HashMap<String, Type>, expected_type: &Type, msg: &str)
               -> TypedAST {
    let tast = f_sub(fundecs, ast, env);
    if ty_of_ast(&tast) != *expected_type {
        panic!(format!("type error in expression {:?}: expected: {:?}, but got: {:?}\n{}", ast, expected_type, ty_of_ast(&tast), msg));
    }
    tast
}

fn f_sub(fundecs: &[FunDec], ast: &AST, env: &mut HashMap<String, Type>)
         -> TypedAST {
    match *ast {
        AST::Num(n) => TypedAST::Num(n),
        AST::Str(ref str) => TypedAST::Str(str.clone()),
        AST::Var(ref x) => {
            let ty = env.get(x).expect("variable not found").clone();
            TypedAST::Var(x.clone(), ty)
        },
        AST::OpNode(op, ref e1, ref e2) => {
            let ta1 = f_sub(fundecs, e1, env);
            let ta2 = f_sub(fundecs, e2, env);
            match (op, ty_of_ast(&ta1), ty_of_ast(&ta2)) {
                (_, Type::Int, Type::Int) =>
                    TypedAST::OpNode(op, Type::Int, Box::new(ta1), Box::new(ta2)),
                (Op::Add, Type::Str, Type::Str) =>
                    TypedAST::OpNode(op, Type::Str, Box::new(ta1), Box::new(ta2)),
                _ => panic!("typing of {:?} failed", op),
            }
        },
        AST::IfNode(ref cond, ref e_true, ref e_false) => {
            let tcond = expect_type(fundecs, cond, env, &Type::Int,
                                    "Condition must be of type int.");
            
            let t_true = f_sub(fundecs, e_true, env);
            let t_false = expect_type(fundecs, e_false, env, &ty_of_ast(&t_true),
                                      "The types of true part and false part in a condition must be the same.");
            TypedAST::IfNode(Box::new(tcond), ty_of_ast(&t_true).clone(), Box::new(t_true), Box::new(t_false))
        },
        AST::LetEx(ref x, ref e1, ref e2) => {
            let ast1 = f_sub(fundecs, e1, env);
            let ty1 = ty_of_ast(&ast1);
            let old = env.insert(x.clone(), ty1.clone());
            let ast2 = f_sub(fundecs, e2, env);
            env.remove(x).unwrap();
            if let Some(o) = old {
                env.insert(x.clone(), o);
            }
            TypedAST::LetEx(x.clone(), ty1, Box::new(ast1), Box::new(ast2))
        },
        AST::FunApp(ref f, ref es) => {
            let sign = get_signature(fundecs, &f);
            let n = es.len(); // # arguments
            let m = sign.0.len(); // # parameters
            if n != m {
                panic!(format!("The number of arguments of {} is wrong. (expected: {}, got: {})", f.clone(), m, n));
            }
            let typed_es = (0 .. n).map(|i| expect_type(fundecs, &es[i], env, &sign.0[i], "Argument type differs.")).collect::<Vec<_>>();
            TypedAST::FunApp(f.clone(), sign.0, sign.1, typed_es)
        },
    }
}

fn get_signature(fundecs: &[FunDec], name: &str) -> (Vec<Type>, Type) {
    let fundec = fundecs.iter().filter(|ref f| f.0 == name).next().expect(&format!("function {} was not found", name));
    (fundec.1.iter().map(|param| param.1.clone()).collect(), fundec.2.clone())
}

pub fn f(fundecs: &[FunDec], ast: &AST) -> (Vec<TypedFunDec>, TypedAST) {
    let tast = f_sub(fundecs, ast, &mut HashMap::new());
    let mut tfundecs: Vec<TypedFunDec> = Vec::new();
    for i in 0 .. fundecs.len() {
        let (fd_name, fd_arg, fd_retty, fd_body) = fundecs[i].clone();
        let mut env = HashMap::new();
        for v_ty in &fd_arg {
            let (ref v, ref ty) = *v_ty;
            env.insert(v.clone(), ty.clone());
        }
        let ty_body = expect_type(fundecs, &fd_body, &mut env, &fd_retty,
                                  "return type differs");
        tfundecs.push((fd_name, fd_arg, fd_retty, ty_body))
    }
    (tfundecs, tast)
}

#[cfg(test)]
mod tests {
    use parse;
    use typing;
    use ast;
    use ast::{AST, Op, Type, TypedAST};
    fn typing(ast: &AST) -> TypedAST {
        typing::f(&Vec::new(), ast).1
    }
    #[test]
    fn operations_test() {
        let ast1 = AST::OpNode(Op::Sub, Box::new(AST::Num(7)), Box::new(AST::Num(4)));
        assert_eq!(ast::ty_of_ast(&typing(&ast1)), Type::Int);
        let ast2 = AST::OpNode(Op::Div, Box::new(AST::Num(20)), Box::new(AST::Num(4)));
        assert_eq!(ast::ty_of_ast(&typing(&ast2)), Type::Int);
    }
    #[test]
    fn letex_test() {
        let ast1 = parse::parse("let x = 4 in x + x");
        assert_eq!(ast::ty_of_ast(&typing(&ast1.1)), Type::Int);
        let ast2 = parse::parse("let x = 4 in let x = 3 in x + x");
        assert_eq!(ast::ty_of_ast(&typing(&ast2.1)), Type::Int);
        let ast3 = parse::parse("let x = 4 in (let x = 3 in x) + x");
        assert_eq!(ast::ty_of_ast(&typing(&ast3.1)), Type::Int);
    }
    #[test]
    fn fundec_test() {
        let (fundec1, ast1) = parse::parse("def id(x: int): int {x} id(5)");
        let (_ty_fundec1, ty_ast1) = typing::f(&fundec1, &ast1);
        assert_eq!(ast::ty_of_ast(&ty_ast1), Type::Int);
    }
}
