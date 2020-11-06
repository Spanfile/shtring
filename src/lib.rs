#![doc(html_root_url = "https://docs.rs/shtring/0.1.0")]
#![warn(clippy::if_not_else)]
#![warn(clippy::needless_pass_by_value)]
#![warn(clippy::non_ascii_literal)]
#![warn(clippy::panic_in_result_fn)]
#![warn(clippy::clippy::too_many_lines)]
#![warn(clippy::clippy::single_match_else)]
#![feature(test)]

//! Split an input string into arguments by whitespace such that text between matching quotes is combined into a single
//! argument. Additionally, single character escapes are supported and ignored where applicable.
//!
//! ```rust
//! # use shtring::{split, Error};
//! # fn main() -> Result<(), Error<'static>> {
//! let input =
//!     "Hello world! \"This text will be a single argument.\" 'So \"will\" this.' \\'Escaped quotes are ignored.\\'";
//! let output = split(input)?;
//! assert_eq!(
//!     output,
//!     vec![
//!         "Hello",
//!         "world!",
//!         "This text will be a single argument.",
//!         "So \"will\" this.",
//!         "\\'Escaped",
//!         "quotes",
//!         "are",
//!         "ignored.\\'",
//!     ]
//! );
//! # Ok(())
//! # }
//! ```
//!
//! The convenience function [split](split) is provided to easily turn an input string into a `Vec` over the parsed
//! arguments, such that if the parser runs into an [error](enum@Error), the parsing is aborted and that error is
//! returned. For other cases, it is possible to create the [Parser](Parser) manually and iterate over the parsed
//! arguments.

mod lexer;
mod parser;

pub use parser::Parser;
use thiserror::Error;

/// The possible error returned from the parser.
#[derive(Debug, Error, Copy, Clone, Eq, PartialEq)]
pub enum Error<'a> {
    /// The input string ended unexpectedly (e.g. there is an unterminated quote or nothing after an escape sequence
    /// starting `\`-character).
    #[error("Unexpected end of input")]
    UnexpectedEndOfInput,
    /// An unexpected token was encountered in the input string (e.g. an unbalanced quote in the middle or at the end
    /// of a word). The error wraps the token's index in the input string and its string value.
    #[error("Unexpected token in input at index {0}: {1}")]
    UnexpectedToken(usize, &'a str),
}

/// Split a given input string into arguments, returning the first encountered error, if any. There may be valid
/// arguments after the erroneous one; if they are desired, use the [Parser](Parser) directly. See the crate-level
/// documentation for an example use.
pub fn split(input: &str) -> Result<Vec<&str>, Error> {
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
