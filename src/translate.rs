use super::ast;
use super::wasm;

pub enum AST {}

#[derive(Clone, Copy)]
pub struct Context {}

trait WASMTranslator<T, U> {
    fn to_wasm(ctx: &Context, node: &T) -> (Context, U);
}

impl WASMTranslator<ast::Program, wasm::Module> for AST {
    fn to_wasm(ctx: &Context, node: &ast::Program) -> (Context, wasm::Module) {
        let (_, mainfunc) = AST::to_wasm(ctx, &node.stmt);
    }
}

impl WASMTranslator<ast::Expr, wasm::Expr> for AST {
    fn to_wasm(ctx: &Context, node: &ast::Expr) -> (Context, wasm::Expr) {
        match node {
            ast::Expr::Block(stmts) => {
                let mut expr = wasm::Expr::new();
                for stmt in stmts {
                    let (_, mut subexpr) = AST::to_wasm(ctx, stmt);
                    expr.append(&mut subexpr);
                }
                (ctx.clone(), expr)
            }
            ast::Expr::BinOp(op, lexpr, rexpr) => {
                let mut expr = wasm::Expr::new();
                let (_, mut lwasm) = AST::to_wasm(ctx, &**lexpr);
                let (_, mut rwasm) = AST::to_wasm(ctx, &**rexpr);
                let (_, mut bowasm) = AST::to_wasm(ctx, op);
                expr.append(&mut lwasm);
                expr.append(&mut rwasm);
                expr.append(&mut bowasm);
                (ctx.clone(), expr)
            }
            ast::Expr::Literal(l) => AST::to_wasm(ctx, l),
            _ => (ctx.clone(), wasm::Expr::new()),
        }
    }
}

impl WASMTranslator<ast::Stmt, wasm::Expr> for AST {
    fn to_wasm(ctx: &Context, node: &ast::Stmt) -> (Context, wasm::Expr) {
        match node {
            ast::Stmt::Expr(e) => AST::to_wasm(ctx, e),
            ast::Stmt::Decl(_, _) => (ctx.clone(), wasm::Expr::new()),
        }
    }
}

impl WASMTranslator<ast::BinOp, wasm::Expr> for AST {
    fn to_wasm(ctx: &Context, node: &ast::BinOp) -> (Context, wasm::Expr) {
        match node {
            ast::BinOp::Add => (ctx.clone(), wasm::Expr::from(wasm::Instr::I64Add)),
            ast::BinOp::Minus => (ctx.clone(), wasm::Expr::from(wasm::Instr::I64Sub)),
            ast::BinOp::Times => (ctx.clone(), wasm::Expr::from(wasm::Instr::I64Mul)),
            ast::BinOp::Divide => (ctx.clone(), wasm::Expr::from(wasm::Instr::I64DivU)),
        }
    }
}

impl WASMTranslator<ast::Literal, wasm::Expr> for AST {
    fn to_wasm(ctx: &Context, node: &ast::Literal) -> (Context, wasm::Expr) {
        match node {
            ast::Literal::Int(u) => (ctx.clone(), wasm::Expr::from(wasm::Instr::I64Const(*u))),
            ast::Literal::Float(z) => (ctx.clone(), wasm::Expr::from(wasm::Instr::F64Const(*z))),
            ast::Literal::Bool(b) => (
                ctx.clone(),
                wasm::Expr::from(wasm::Instr::I64Const(*b as u64)),
            ),
        }
    }
}
