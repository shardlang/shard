use super::token_kind::TokenKind;

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum Precedence {
    Lowest,
    Add,   // +
    Times, // *
}

impl Precedence {
    pub fn from(kind: &TokenKind) -> Precedence {
        match kind {
            TokenKind::Plus | TokenKind::Minus => Precedence::Add,
            TokenKind::Star | TokenKind::Slash => Precedence::Times,
            _ => Precedence::Lowest,
        }
    }
}
