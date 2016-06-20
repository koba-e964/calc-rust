use ast::{AST, Type, TypedAST, Op, ty_of_ast};
use std::collections::HashMap;

fn f_sub(ast: &AST, env: &mut HashMap<String, Type>) -> TypedAST {
    match *ast {
        AST::Num(n) => TypedAST::Num(n),
        AST::Str(ref str) => TypedAST::Str(str.clone()),
        AST::Var(ref x) => panic!("typing::f: typing var"),
        AST::OpNode(op, ref e1, ref e2) => {
            let ta1 = f_sub(e1, env);
            let ta2 = f_sub(e2, env);
            match (op, ty_of_ast(&ta1), ty_of_ast(&ta2)) {
                (_, Type::Int, Type::Int) =>
                    TypedAST::OpNode(op, Type::Int, Box::new(ta1), Box::new(ta2)),
                (Op::Add, Type::Str, Type::Str) =>
                    TypedAST::OpNode(op, Type::Str, Box::new(ta1), Box::new(ta2)),
                _ => panic!("typing of {:?} failed", op),
            }
        },
        AST::LetEx(_, _, _) => panic!("typing::f: typing let expression"),
    }
}

pub fn f(ast: &AST) -> TypedAST {
    f_sub(ast, &mut HashMap::new())
}
