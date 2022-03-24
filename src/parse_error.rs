use std::fmt;

pub type ParseResult<T> = std::result::Result<T, ParseError>;

#[derive(Debug)]
pub struct ParseError;

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Parsing Error.")
    }
}
