// AST
#[derive(PartialEq, Clone, Debug)]
pub enum AST {
    Num(i64),
    Str(String),
    Var(String),
    OpNode(Op, Box<AST>, Box<AST>),
    IfNode(Box<AST>, Box<AST>, Box<AST>), // If the first evaluates to non-zero, return the second. Otherwise, return the third.
    LetEx(String, Box<AST>, Box<AST>),
    FunApp(String, Vec<AST>),
}

pub type FunDec = (String, Vec<(String, Type)>, Type, AST);

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Op {
    Add, Sub,
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
    OpNode(Op, Type, Box<TypedAST>, Box<TypedAST>),
    IfNode(Box<TypedAST>, Type, Box<TypedAST>, Box<TypedAST>),
    LetEx(String, Type, Box<TypedAST>, Box<TypedAST>),
    FunApp(String, Vec<Type>, Type, Vec<TypedAST>), // name, argtype, rettype, arg
}

pub type TypedFunDec = (String, Vec<(String, Type)>, Type, TypedAST);

pub fn ty_of_ast(tast: &TypedAST) -> Type {
    match *tast {
        TypedAST::Num(_) => Type::Int,
        TypedAST::Str(_) => Type::Str,
        TypedAST::Var(_, ref ty) => ty.clone(),
        TypedAST::OpNode(_, ref ty, _, _) => ty.clone(),
        TypedAST::IfNode(_, ref ty, _, _) => ty.clone(),
        TypedAST::LetEx(_, ref ty, _, _) => ty.clone(),
        TypedAST::FunApp(_, _, ref ty, _) => ty.clone(),
    }
}
