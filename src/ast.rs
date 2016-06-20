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

#[derive(PartialEq, Clone, Debug)]
pub enum Value {
    VNum(i64),
    VStr(String),
}

/*
 * Copy trait cannot be implemented because we might add some recursive constructors.
 */
#[derive(PartialEq, Clone, Debug)]
pub enum Type {
    Int,
    Str,
}

#[derive(PartialEq, Clone, Debug)]
pub enum TypedAST {
    Num(i64),
    Str(String),
    Var(String, Type),
    AddNode(AddOp, Type, Box<TypedAST>, Box<TypedAST>),
    MulNode(MulOp, Type, Box<TypedAST>, Box<TypedAST>),
    LetEx(String, Type, Box<TypedAST>, Box<TypedAST>),
}

fn ty_of_ast(tast: &TypedAST) -> Type {
    match *tast {
        TypedAST::Num(_) => Type::Int,
        TypedAST::Str(_) => Type::Str,
        TypedAST::Var(_, ref ty) => ty.clone(),
        TypedAST::AddNode(_, ref ty, _, _) => ty.clone(),
        TypedAST::MulNode(_, ref ty, _, _) => ty.clone(),
        TypedAST::LetEx(_, ref ty, _, _) => ty.clone(),
    }
}
