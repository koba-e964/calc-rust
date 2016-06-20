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

#[cfg(test)]
mod tests {
    use parse;
    use typing;
    use ast;
    use ast::{AST, Op, Type};
    #[test]
    fn operations_test() {
        let ast1 = AST::OpNode(Op::Sub, Box::new(AST::Num(7)), Box::new(AST::Num(4)));
        assert_eq!(ast::ty_of_ast(&typing::f(&ast1)), Type::Int);
        let ast2 = AST::OpNode(Op::Div, Box::new(AST::Num(20)), Box::new(AST::Num(4)));
        assert_eq!(ast::ty_of_ast(&typing::f(&ast2)), Type::Int);
    }
    #[test]
    fn letex_test() {
        let ast1 = parse::parse("let x = 4 in x + x");
        assert_eq!(ast::ty_of_ast(&typing::f(&ast1)), Type::Int);
        let ast2 = parse::parse("let x = 4 in let x = 3 in x + x");
        assert_eq!(ast::ty_of_ast(&typing::f(&ast2)), Type::Int);
        let ast3 = parse::parse("let x = 4 in (let x = 3 in x) + x");
        assert_eq!(ast::ty_of_ast(&typing::f(&ast3)), Type::Int);
    }
}
