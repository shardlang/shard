mod ast;
mod parse_error;
mod parser;
mod precedence;
mod token;
mod token_buffer;
mod token_kind;
mod translate;
mod util;
mod wasm;

use logos::Logos;
use parser::Parser;
use std::{env, fs};
use token_kind::TokenKind;

fn main() {
    let mut dir = env::current_dir().unwrap();
    dir.push("test.sd");
    println!("{:?}", dir.as_os_str());
    let bytes = fs::read(dir).unwrap();
    let foo = String::from_utf8_lossy(bytes.as_slice());
    let lex = TokenKind::lexer(&foo);
    let mut parser = Parser::new(lex);
    parser.exhaust();
    println!("{}", parser);
    let expr = parser.parse_program().unwrap();

    println!("{:?}", &expr);
}
