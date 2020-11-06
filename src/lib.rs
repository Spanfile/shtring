#![warn(clippy::if_not_else)]
#![warn(clippy::needless_pass_by_value)]
#![warn(clippy::non_ascii_literal)]
#![warn(clippy::panic_in_result_fn)]
#![warn(clippy::clippy::too_many_lines)]
#![warn(clippy::clippy::single_match_else)]

mod lexer;

use thiserror::Error;

#[derive(Debug, Error, Copy, Clone, Eq, PartialEq)]
pub enum Error {
    #[error("Unexpected end of input")]
    UnexpectedEndOfInput,
    #[error("Unsupported character")]
    UnsupportedCharacter,
}

#[cfg(test)]
mod tests {}
