use crate::ast::Token;
use std::{iter::Peekable, str::Chars};

#[derive(Debug)]
pub(crate) struct Lexer<'lex> {
    chr_strm: Peekable<Chars<'lex>>,
}

impl<'lex> Lexer<'lex> {
    pub(crate) fn new(input: &'lex mut String) -> Self {
        // Removes whitespace
        *input = input
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>();

        Self {
            chr_strm: input.chars().peekable(),
        }
    }

    fn peek_char(&mut self) -> Option<&char> {
        self.chr_strm.peek()
    }

    fn next_char(&mut self) -> char {
        self.chr_strm.next().unwrap()
    }

    // Returns the entire number sequence as a string while advancing the iter
    fn get_num(&mut self) -> String {
        let mut num = String::new();
        num.push(self.next_char());

        while let Some(chr) = self.peek_char() {
            if chr.is_ascii_digit() {
                num.push(self.next_char());
            } else {
                break;
            }
        }

        num
    }
}

impl Iterator for Lexer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.peek_char().is_none() {
            // TODO: Consider removing Token::Eos in favor of using None to represent the end of the stream.
            return Some(Token::Eos);
        }

        if let Some(chr) = self.peek_char() {
            if chr.is_ascii_digit() {
                return Some(Token::build_num_token(self.get_num()));
            }
        }

        Some(Token::build_token(self.next_char()))
    }
}
