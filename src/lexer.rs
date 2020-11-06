use crate::Error;
use std::{iter::Peekable, str::CharIndices};

#[derive(Debug)]
pub(crate) struct Lexer<'a> {
    input: &'a str,
    chars: Peekable<CharIndices<'a>>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(crate) enum Token<'a> {
    Word(&'a str),
    Whitespace,
    SingleQuote,
    DoubleQuote,
    Escape(&'a str),
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            chars: input.char_indices().peekable(),
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<(usize, Token<'a>), Error>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.chars.next() {
            Some((idx, chr)) => match chr {
                '\'' => Some(Ok((idx, Token::SingleQuote))),
                '"' => Some(Ok((idx, Token::DoubleQuote))),
                '\\' => match self.chars.next() {
                    Some((cont, _)) => Some(Ok((idx, Token::Escape(&self.input[(idx..cont + 1)])))),
                    None => Some(Err(Error::UnexpectedEndOfInput)),
                },
                c if c.is_whitespace() => {
                    loop {
                        match self.chars.peek() {
                            Some((_, c)) if c.is_whitespace() => self.chars.next(),
                            _ => break,
                        };
                    }
                    Some(Ok((idx, Token::Whitespace)))
                }
                c if is_word_character(c) => {
                    let mut end = idx;
                    loop {
                        match self.chars.peek() {
                            Some((cont, c)) if is_word_character(*c) => end = *cont,
                            _ => break,
                        }
                        self.chars.next();
                    }
                    Some(Ok((idx, Token::Word(&self.input[(idx..end + 1)]))))
                }
                _ => Some(Err(Error::UnsupportedCharacter)),
            },
            None => None,
        }
    }
}

fn is_word_character(c: char) -> bool {
    c != '\'' && c != '"' && c != '\\' && !c.is_whitespace()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_character_word_token() {
        let input = "a";
        let mut lexer = Lexer::new(input);
        let output = lexer.next();
        assert_eq!(output, Some(Ok((0, Token::Word(input)))));
    }

    #[test]
    fn word_token() {
        let input = "hello";
        let mut lexer = Lexer::new(input);
        let output = lexer.next();
        assert_eq!(output, Some(Ok((0, Token::Word(input)))));
    }

    #[test]
    fn single_quote_token() {
        let input = "'";
        let mut lexer = Lexer::new(input);
        let output = lexer.next();
        assert_eq!(output, Some(Ok((0, Token::SingleQuote))));
    }

    #[test]
    fn double_quote_token() {
        let input = "\"";
        let mut lexer = Lexer::new(input);
        let output = lexer.next();
        assert_eq!(output, Some(Ok((0, Token::DoubleQuote))));
    }

    #[test]
    fn single_whitespace_token() {
        let input = " ";
        let mut lexer = Lexer::new(input);
        let output = lexer.next();
        assert_eq!(output, Some(Ok((0, Token::Whitespace))));
    }

    #[test]
    fn multiple_whitespace_tokens() {
        let input = " \t";
        let mut lexer = Lexer::new(input);
        let output = lexer.next();
        assert_eq!(output, Some(Ok((0, Token::Whitespace))));
    }

    #[test]
    fn escape_token() {
        let input = r"\t";
        let mut lexer = Lexer::new(input);
        let output = lexer.next();
        assert_eq!(output, Some(Ok((0, Token::Escape(input)))));
    }

    #[test]
    fn unexpected_eoi() {
        let input = r"\";
        let mut lexer = Lexer::new(input);
        let output = lexer.next();
        assert_eq!(output, Some(Err(Error::UnexpectedEndOfInput)));
    }

    #[test]
    fn multiple_tokens() {
        let input = "'hello,' \"world!\" \\\"";
        let lexer = Lexer::new(input);
        let output: Vec<Result<(usize, Token<'_>), Error>> = lexer.collect();
        assert_eq!(
            output,
            vec![
                Ok((0, Token::SingleQuote)),
                Ok((1, Token::Word("hello,"))),
                Ok((7, Token::SingleQuote)),
                Ok((8, Token::Whitespace)),
                Ok((9, Token::DoubleQuote)),
                Ok((10, Token::Word("world!"))),
                Ok((16, Token::DoubleQuote)),
                Ok((17, Token::Whitespace)),
                Ok((18, Token::Escape("\\\"")))
            ]
        );
    }
}
