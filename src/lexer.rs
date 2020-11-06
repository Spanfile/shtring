use crate::Error;
use std::{fmt, fmt::Display, iter::Peekable, str::CharIndices};

#[derive(Debug)]
pub(crate) struct Lexer<'a> {
    input: &'a str,
    chars: Peekable<CharIndices<'a>>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(crate) enum Token<'a> {
    Word(&'a str),
    Whitespace(&'a str),
    SingleQuote,
    DoubleQuote,
    Escape(&'a str),
    UnknownCharacter(char),
}

impl Token<'_> {
    pub fn len(self) -> usize {
        match self {
            Token::Word(w) | Token::Whitespace(w) | Token::Escape(w) => w.len(),
            Token::UnknownCharacter(c) => c.len_utf8(),
            Token::SingleQuote | Token::DoubleQuote => 1,
        }
    }
}

impl Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Word(w) | Token::Whitespace(w) | Token::Escape(w) => write!(f, "{}", w),
            Token::UnknownCharacter(c) => write!(f, "{}", c),
            Token::SingleQuote => write!(f, "'"),
            Token::DoubleQuote => write!(f, "\""),
        }
    }
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
    type Item = Result<(usize, Token<'a>), Error<'a>>;

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
                    let mut end = idx;
                    loop {
                        match self.chars.peek() {
                            Some((cont, c)) if c.is_whitespace() => end = *cont,
                            _ => break,
                        }
                        self.chars.next();
                    }
                    Some(Ok((idx, Token::Whitespace(&self.input[(idx..end + 1)]))))
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
                c => Some(Ok((idx, Token::UnknownCharacter(c)))),
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
    extern crate test;

    use super::*;
    use test::Bencher;

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
        assert_eq!(output, Some(Ok((0, Token::Whitespace(input)))));
    }

    #[test]
    fn multiple_whitespace_tokens() {
        let input = " \t";
        let mut lexer = Lexer::new(input);
        let output = lexer.next();
        assert_eq!(output, Some(Ok((0, Token::Whitespace(input)))));
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

    #[bench]
    fn multiple_tokens(b: &mut Bencher) {
        b.iter(|| {
            let input = "'hello,' \"world!\" \\\"";
            let lexer = Lexer::new(input);
            let output: Vec<Result<(usize, Token<'_>), Error>> = lexer.collect();
            assert_eq!(
                output,
                vec![
                    Ok((0, Token::SingleQuote)),
                    Ok((1, Token::Word("hello,"))),
                    Ok((7, Token::SingleQuote)),
                    Ok((8, Token::Whitespace(" "))),
                    Ok((9, Token::DoubleQuote)),
                    Ok((10, Token::Word("world!"))),
                    Ok((16, Token::DoubleQuote)),
                    Ok((17, Token::Whitespace(" "))),
                    Ok((18, Token::Escape("\\\"")))
                ]
            );
        });
    }
}
