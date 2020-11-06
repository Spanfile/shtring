use crate::{
    lexer::{Lexer, Token},
    Error,
};

/// Iterator over the arguments in an input string.
///
/// The individual returned items for an input string `&'a str` are `Result<&'a str, Error>` (see
/// [error-handling](#error-handling) below for notes on the individual results). Escape sequences in the format
/// `\<character>` are parsed as normal characters. The iterator will return `None` once the input has been exhausted.
///
/// ```rust
/// # use shtring::Parser;
/// let input = "a \"b c\" \\\"d";
/// let mut parser = Parser::new(input);
/// assert_eq!(parser.next(), Some(Ok("a")));
/// assert_eq!(parser.next(), Some(Ok("b c")));
/// assert_eq!(parser.next(), Some(Ok("\\\"d")));
/// assert_eq!(parser.next(), None);
/// ```
///
/// # Error handling
///
/// The parser will recover from any errors encountered while parsing individual arguments. This means that if some
/// argument fails to be parsed, there still may be more valid arguments after it, in the sense that the erroneous
/// argument was ignored.
///
/// ```rust
/// # use shtring::{Error, Parser};
/// let input = "a b\" c";
/// let mut parser = Parser::new(input);
/// assert_eq!(parser.next(), Some(Ok("a")));
/// assert_eq!(parser.next(), Some(Err(Error::UnexpectedToken(3, "\""))));
/// assert_eq!(parser.next(), Some(Ok("c")));
/// assert_eq!(parser.next(), None);
/// ```
#[derive(Debug)]
pub struct Parser<'a> {
    input: &'a str,
    lexer: Lexer<'a>,
}

impl<'a> Parser<'a> {
    /// Return a new [Parser](Parser) over a given input string.
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            lexer: Lexer::new(input),
        }
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = Result<&'a str, Error<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            break match self.lexer.next() {
                Some(Ok((idx, token))) => match token {
                    Token::Whitespace(_) => continue,
                    Token::Word(_) | Token::UnknownCharacter(_) | Token::Escape(_) => loop {
                        match self.lexer.next() {
                            Some(Ok((cont, Token::Whitespace(_)))) => break Some(Ok(&self.input[(idx..cont)])),
                            Some(Ok((_, Token::Word(_))))
                            | Some(Ok((_, Token::UnknownCharacter(_))))
                            | Some(Ok((_, Token::Escape(_)))) => continue,
                            Some(Ok((cont, token))) => {
                                break Some(Err(Error::UnexpectedToken(cont, &self.input[cont..cont + token.len()])))
                            }
                            Some(Err(e)) => break Some(Err(e)),
                            None => break Some(Ok(&self.input[(idx..)])),
                        }
                    },
                    Token::SingleQuote | Token::DoubleQuote => loop {
                        match self.lexer.next() {
                            Some(Ok((cont, quote))) if quote == token => break Some(Ok(&self.input[idx + 1..cont])),
                            Some(Ok((_, _))) => continue,
                            Some(Err(Error::UnexpectedEndOfInput)) | None => {
                                break Some(Err(Error::UnexpectedEndOfInput))
                            }
                            Some(Err(e)) => break Some(Err(e)),
                        }
                    },
                },
                Some(Err(e)) => Some(Err(e)),
                None => None,
            };
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::*;
    use test::Bencher;

    #[test]
    fn single_word() {
        let input = "a";
        let mut parser = Parser::new(input);
        let output = parser.next();
        assert_eq!(output, Some(Ok(input)));
    }

    #[test]
    fn multiple_words() {
        let input = "a b c";
        let parser = Parser::new(input);
        let output: Vec<Result<&str, Error>> = parser.collect();
        assert_eq!(output, vec![Ok("a"), Ok("b"), Ok("c")]);
    }

    #[test]
    fn single_quoted_word() {
        let input = "'a b c'";
        let parser = Parser::new(input);
        let output: Vec<Result<&str, Error>> = parser.collect();
        assert_eq!(output, vec![Ok("a b c")]);
    }

    #[test]
    fn double_quoted_word() {
        let input = "\"a b c\"";
        let parser = Parser::new(input);
        let output: Vec<Result<&str, Error>> = parser.collect();
        assert_eq!(output, vec![Ok("a b c")]);
    }

    #[test]
    fn escaped_quote() {
        let input = "\\\"a";
        let mut parser = Parser::new(input);
        let output = parser.next();
        assert_eq!(output, Some(Ok(input)));
    }

    #[test]
    fn escaped_quotes() {
        let input = "\\\" a \\\"";
        let parser = Parser::new(input);
        let output: Vec<Result<&str, Error>> = parser.collect();
        assert_eq!(output, vec![Ok("\\\""), Ok("a"), Ok("\\\"")]);
    }

    #[test]
    fn unterminated_single_quote() {
        let input = "'a";
        let mut parser = Parser::new(input);
        let output = parser.next();
        assert_eq!(output, Some(Err(Error::UnexpectedEndOfInput)));
    }

    #[test]
    fn unterminated_double_quote() {
        let input = "\"a";
        let mut parser = Parser::new(input);
        let output = parser.next();
        assert_eq!(output, Some(Err(Error::UnexpectedEndOfInput)));
    }

    #[test]
    fn mismatched_quote() {
        let input = "\"a'";
        let mut parser = Parser::new(input);
        let output = parser.next();
        assert_eq!(output, Some(Err(Error::UnexpectedEndOfInput)));
    }

    #[test]
    fn unexpected_quote() {
        let input = "a\"";
        let mut parser = Parser::new(input);
        let output = parser.next();
        assert_eq!(output, Some(Err(Error::UnexpectedToken(1, "\""))));
    }

    #[bench]
    fn multiple_words_with_escapes_and_quotes(b: &mut Bencher) {
        b.iter(|| {
            let input = "a \"b \\\"c d\" e 'f g'";
            let parser = Parser::new(input);
            let output: Vec<Result<&str, Error>> = parser.collect();
            assert_eq!(output, vec![Ok("a"), Ok("b \\\"c d"), Ok("e"), Ok("f g")]);
        });
    }
}
