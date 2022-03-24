use logos::Lexer;
use std::ops::Range;

use super::token_kind::TokenKind;

#[derive(Clone)]
pub struct Token {
    _kind: TokenKind,
    _span: Range<usize>,
    _slice: String,
}

impl Token {
    pub fn pop_from<'a>(lexer: &mut Lexer<'a, TokenKind>) -> Option<Token> {
        match lexer.next() {
            Some(tkn) => Some(Token {
                _kind: tkn,
                _span: lexer.span(),
                _slice: lexer.slice().to_string(),
            }),
            None => None,
        }
    }
    pub fn eof<'a>(lexer: &Lexer<'a, TokenKind>) -> Token {
        Token {
            _kind: TokenKind::Eof,
            _span: lexer.span(),
            _slice: String::new(),
        }
    }

    pub fn kind(&self) -> TokenKind {
        self._kind
    }
    pub fn source_str(&self) -> String {
        self._slice.clone()
    }
}
