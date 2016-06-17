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
