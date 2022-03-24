pub type Ident = String;

#[derive(Debug)]
pub struct Program {
    pub stmt: Stmt,
}

#[derive(Debug)]
pub enum Stmt {
    Expr(Expr),
    Decl(Ident, Expr),
}

#[derive(Debug)]
pub enum BinOp {
    Add,
    Minus,
    Times,
    Divide,
}

#[derive(Debug)]
pub enum Expr {
    Block(Vec<Stmt>),
    BinOp(BinOp, Box<Expr>, Box<Expr>),
    Literal(Literal),
    Fn(Vec<Arg>, Box<Expr>),
}

#[derive(Debug)]
pub enum Literal {
    Int(u64),
    Float(f64),
    //String(String),
    Bool(bool),
}

#[derive(Debug)]
pub struct Arg {
    pub name: Ident,
    pub typename: Ident,
}
