use super::ast::*;
use super::parse_error::{ParseError, ParseResult};
use super::precedence::Precedence;
use super::token_buffer::TokenBuffer;
use super::token_kind::TokenKind;
use logos::Lexer;
use std::fmt;

pub struct Parser<'a> {
    tokens: TokenBuffer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a, TokenKind>) -> Parser {
        Parser {
            tokens: TokenBuffer::new(lexer),
        }
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.tokens.get_token_kind(0) {
                TokenKind::Whitespace => self.tokens.bump(),
                _ => break,
            }
        }
    }

    pub fn parse_block_expr(&mut self) -> ParseResult<Expr> {
        self.skip_whitespace();
        self.tokens.assert_token(0, TokenKind::LeftBrace)?;
        self.skip_whitespace();
        let mut stmt_list = Vec::new();
        loop {
            match self.tokens.get_token_kind(0) {
                TokenKind::Semicolon | TokenKind::Newline => {
                    self.tokens.bump();
                    self.skip_whitespace();
                }
                TokenKind::RightBrace => {
                    self.tokens.bump();
                    self.skip_whitespace();
                    return Ok(Expr::Block(stmt_list));
                }
                _ => {
                    let stmt = self.parse_stmt()?;
                    stmt_list.push(stmt);
                }
            }
        }
    }

    pub fn parse_program(&mut self) -> ParseResult<Program> {
        let stmt = self.parse_stmt()?;
        Ok(Program { stmt })
    }

    pub fn parse_stmt(&mut self) -> ParseResult<Stmt> {
        match self.tokens.get_token_kind(0) {
            TokenKind::Let => self.parse_decl_stmt(),
            _ => self.parse_expr_stmt(),
        }
    }

    pub fn parse_decl_stmt(&mut self) -> ParseResult<Stmt> {
        self.skip_whitespace();
        self.tokens.assert_token(0, TokenKind::Let)?;
        let var_name = self.parse_ident()?;
        self.tokens.assert_token(0, TokenKind::Eq)?;
        let expr = self.parse_expr(Precedence::Lowest)?;
        Ok(Stmt::Decl(var_name, expr))
    }

    pub fn parse_expr_stmt(&mut self) -> ParseResult<Stmt> {
        let expr = self.parse_expr(Precedence::Lowest)?;
        Ok(Stmt::Expr(expr))
    }

    pub fn parse_ident(&mut self) -> ParseResult<Ident> {
        self.skip_whitespace();
        let tkn = self.tokens.assert_token(0, TokenKind::Ident)?;
        self.skip_whitespace();
        Ok(tkn.source_str())
    }

    pub fn parse_typename(&mut self) -> ParseResult<Ident> {
        self.parse_ident()
    }

    pub fn parse_arg(&mut self) -> ParseResult<Arg> {
        self.skip_whitespace();
        let name = self.parse_ident()?;
        self.skip_whitespace();
        self.tokens.assert_token(0, TokenKind::Colon)?;
        self.skip_whitespace();
        let typename = self.parse_typename()?;
        self.skip_whitespace();
        Ok(Arg { name, typename })
    }

    pub fn parse_arg_list(&mut self) -> ParseResult<Vec<Arg>> {
        self.skip_whitespace();
        let mut arg_list = Vec::new();
        loop {
            match self.parse_arg() {
                Ok(arg) => {
                    arg_list.push(arg);
                    match self.tokens.get_token_kind(0) {
                        TokenKind::Comma => self.tokens.bump(),
                        _ => return Ok(arg_list),
                    }
                }
                Err(_) => return Ok(arg_list),
            }
        }
    }

    pub fn parse_fn(&mut self) -> ParseResult<Expr> {
        self.skip_whitespace();
        self.tokens.assert_token(0, TokenKind::LeftParen)?;
        let args = self.parse_arg_list()?;
        self.tokens.assert_token(0, TokenKind::RightParen)?;
        self.skip_whitespace();
        self.tokens.assert_token(0, TokenKind::Eq)?;
        self.tokens.assert_token(0, TokenKind::Gt)?;
        let expr = self.parse_expr(Precedence::Lowest)?;
        Ok(Expr::Fn(args, Box::new(expr)))
    }

    pub fn parse_expr(&mut self, precedence: Precedence) -> ParseResult<Expr> {
        self.skip_whitespace();
        let prefix = self.tokens.get_token(0);
        let mut prev = match prefix.kind() {
            TokenKind::LeftBrace => self.parse_block_expr()?,
            TokenKind::LeftParen => {
                let snap = self.tokens.save_snap();
                match self.parse_fn() {
                    Ok(expr) => expr,
                    Err(_) => {
                        self.tokens.return_snap(snap);
                        self.parse_grouped_expr()?
                    }
                }
            }
            TokenKind::Int => {
                let val = prefix.source_str().parse().expect("Not an integer?");
                let res = Expr::Literal(Literal::Int(val));
                self.tokens.bump();
                res
            }
            _ => return Err(ParseError),
        };

        loop {
            self.skip_whitespace();
            let next_token = self.tokens.get_token(0);
            let kind = next_token.kind();
            match kind {
                TokenKind::Plus | TokenKind::Minus | TokenKind::Star | TokenKind::Slash => {
                    if Precedence::from(&kind) > precedence {
                        prev = self.parse_binop_expr(prev)?
                    } else {
                        return Ok(prev);
                    }
                }
                _ => return Ok(prev),
            }
        }
    }

    pub fn parse_grouped_expr(&mut self) -> ParseResult<Expr> {
        self.tokens.assert_token(0, TokenKind::LeftParen)?;
        let expr = self.parse_expr(Precedence::Lowest);
        self.tokens.assert_token(0, TokenKind::RightParen)?;
        expr
    }

    pub fn parse_binop_expr(&mut self, prev: Expr) -> ParseResult<Expr> {
        self.skip_whitespace();
        let prefix = self.tokens.get_token(0);
        let kind = prefix.kind();
        let op = match kind {
            TokenKind::Plus => BinOp::Add,
            TokenKind::Minus => BinOp::Minus,
            TokenKind::Star => BinOp::Times,
            TokenKind::Slash => BinOp::Divide,
            _ => return Err(ParseError),
        };
        self.tokens.bump();
        self.skip_whitespace();
        let r_expr = self.parse_expr(Precedence::from(&kind))?;
        Ok(Expr::BinOp(op, Box::new(prev), Box::new(r_expr)))
    }

    pub fn exhaust(&mut self) {
        self.tokens.exhaust()
    }
}

impl fmt::Display for Parser<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.tokens.fmt(f)
    }
}
