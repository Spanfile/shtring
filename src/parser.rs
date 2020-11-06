use crate::{
    lexer::{Lexer, Token},
    Error,
};

#[derive(Debug)]
pub struct Parser<'a> {
    input: &'a str,
    lexer: Lexer<'a>,
}

impl<'a> Parser<'a> {
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
