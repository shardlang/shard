use super::parse_error::{ParseError, ParseResult};
use super::token::Token;
use super::token_kind::TokenKind;
use logos::Lexer;
use std::fmt;

pub struct TokenBuffer<'a> {
    lexer: Lexer<'a, TokenKind>,
    tokens: Vec<Token>,
    offset: usize,
    eof: Option<Token>,
}

impl<'a> TokenBuffer<'a> {
    pub fn new(lexer: Lexer<'a, TokenKind>) -> Self {
        TokenBuffer {
            lexer,
            tokens: Vec::with_capacity(10),
            offset: 0,
            eof: None,
        }
    }

    fn get_token_global(&mut self, index: usize) -> &Token {
        let len = self.tokens.len();
        if index >= len {
            for _ in len..(index + 1) {
                match Token::pop_from(&mut self.lexer) {
                    Some(tkn) => self.tokens.push(tkn),
                    None => self.eof = Some(Token::eof(&self.lexer)),
                }
            }
        }
        match self.tokens.get(index) {
            Some(tkn) => tkn,
            None => self.eof.as_ref().unwrap(),
        }
    }

    pub fn get_token(&mut self, index: usize) -> &Token {
        self.get_token_global(index + self.offset)
    }

    pub fn get_token_kind(&mut self, index: usize) -> TokenKind {
        self.get_token(index).kind()
    }

    pub fn assert_token(&mut self, index: usize, kind: TokenKind) -> ParseResult<Token> {
        let tkn = self.get_token(index).clone();
        if tkn.kind() == kind {
            self.bump();
            Ok(tkn)
        } else {
            Err(ParseError)
        }
    }

    pub fn bump(&mut self) {
        self.offset += 1;
    }

    pub fn save_snap(&mut self) -> usize {
        self.offset
    }

    pub fn return_snap(&mut self, snap: usize) {
        self.offset = snap
    }

    pub fn exhaust(&mut self) {
        loop {
            match Token::pop_from(&mut self.lexer) {
                Some(tkn) => self.tokens.push(tkn),
                None => {
                    self.eof = Some(Token::eof(&self.lexer));
                    break;
                }
            }
        }
    }
}

impl fmt::Display for TokenBuffer<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        let mut first = true;
        for tkn in &self.tokens {
            let kind = tkn.kind();
            match kind {
                TokenKind::Eof => break,
                _ => {
                    let str = if first {
                        format!("{:?}", kind)
                    } else {
                        format!(", {:?}", kind)
                    };
                    f.write_str(&str)?;
                }
            }
            first = false;
        }
        write!(f, "]")
    }
}
