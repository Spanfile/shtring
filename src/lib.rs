#![warn(clippy::if_not_else)]
#![warn(clippy::needless_pass_by_value)]
#![warn(clippy::non_ascii_literal)]
#![warn(clippy::panic_in_result_fn)]
#![warn(clippy::clippy::too_many_lines)]
#![warn(clippy::clippy::single_match_else)]
#![feature(test)]

mod lexer;
mod parser;

pub use parser::Parser;
use thiserror::Error;

#[derive(Debug, Error, Copy, Clone, Eq, PartialEq)]
pub enum Error<'a> {
    #[error("Unexpected end of input")]
    UnexpectedEndOfInput,
    #[error("Unexpected token in input at index {0}: {1}")]
    UnexpectedToken(usize, &'a str),
}

pub fn split<'a>(input: &'a str) -> Result<Vec<&'a str>, Error> {
    Parser::new(input).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_split() {
        let input = "a \"b c\"";
        let output = split(input);
        assert_eq!(output, Ok(vec!["a", "b c"]))
    }

    #[test]
    fn invalid_split_unexpected_eoi() {
        let input = "a \"b c";
        let output = split(input);
        assert_eq!(output, Err(Error::UnexpectedEndOfInput));
    }

    #[test]
    fn invalid_split_unexpected_token() {
        let input = "a b c\"";
        let output = split(input);
        assert_eq!(output, Err(Error::UnexpectedToken(5, "\"")));
    }
}
