use crate::*;
use logos::Logos;
use std::iter::Peekable;
use std::vec::IntoIter;

type ParsingLexer = Peekable<IntoIter<(Result<TokenKind, ()>, std::ops::Range<usize>)>>;

pub struct Parser<'a> {
    pub file: String,
    pub lexer: ParsingLexer,
    pub input: &'a str,
    pub errors: Vec<ParserError>,
}

impl<'a> Parser<'a> {
    pub fn new(file: String, input: &'a str) -> Result<Self, Vec<ParserError>> {
        let errors = Vec::new();
        let lexer = TokenKind::lexer(input).spanned();

        let first_pass_tokens = Self::first_pass(file.to_string(), input.to_string(), lexer);
        let toks = match first_pass_tokens {
            Err(e) => return Err(e),
            Ok(ref v) => v,
        };
        let second_pass_tokens = Self::second_pass(
            &mut Parser {
                file: file.to_string(),
                lexer: toks.clone().into_iter().peekable(),
                input,
                errors: Vec::new(),
            },
            first_pass_tokens?,
        );
        Ok(Parser {
            file,
            lexer: second_pass_tokens.into_iter().peekable(),
            input,
            errors,
        })
    }
    pub fn parse(&mut self) -> Result<Vec<(TokenKind, std::ops::Range<usize>)>, &Vec<ParserError>> {
        let mut tokens = Vec::new();

        while let Some((token, span)) = self.lexer.next() {
            match token {
                Ok(TokenKind::Whitespace) | Ok(TokenKind::Tab) => {}
                Ok(TokenKind::MacroDef(_)) => tokens.extend(self.parse_single_macro()),
                Ok(t) => {
                    tokens.push((t, span));
                }
                Err(()) => {
                    self.errors.push(ParserError {
                        file: self.file.to_string(),
                        help: Some(String::from(
                            "this is likely an internal error, please report it",
                        )),
                        input: self.input.to_string(),
                        message: "Unknown error encountered whilst parsing".to_string(),
                        start_pos: span.start,
                        last_pos: span.end,
                    });
                }
            }
        }

        if !self.errors.is_empty() {
            return Err(&self.errors);
        }

        Ok(tokens)
    }
}
