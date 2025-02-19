use crate::*;
use logos::Logos;
use std::fmt;

#[derive(Debug)]
pub struct LexerError {
    pub input: String,
    pub message: String,
    pub line: usize,
    pub column: usize,
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "at line {}:{} - {}",
            self.line, self.column, self.message
        )?;
        let mut nlc = 0;
        for (index, character) in self.input.chars().enumerate() {
            if character == '\n' {
                nlc += 1;
            }
            if index == self.line {
                break;
            }
        }
        let mut linedata = self.input.lines();
        for _ in 0..nlc {
            linedata.next();
        }
        writeln!(f, "{}", linedata.next().unwrap())?;
        write!(
            f,
            "{}{}",
            " ".repeat(self.line),
            "^".repeat(self.column - self.line)
        )
    }
}
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

fn lex_single_macro_argument(
    arg_name: String,
    input: &str,
    lexer: &mut logos::SpannedIter<TokenKind>,
) -> Result<Vec<FullArgument>, LexerError> {
    // teehee,
    // found an argument
    let input_str = input.to_string();
    let (val, _) = match lexer.next() {
        Some((v, l)) => (v, l),
        None => panic!(),
    };
    let mut args = Vec::new();
    match val {
        Ok(TokenKind::Colon) => {
            let (val, loc) = match lexer.next() {
                Some((v, l)) => (v, l),
                None => panic!(),
            };
            match val {
                Ok(TokenKind::Ident(arg_type_str)) => {
                    let arg_type =
                        ArgumentType::from_str(&arg_type_str).ok_or_else(|| LexerError {
                            input: input.to_string(),
                            message: format!("Invalid argument type: {}", arg_type_str),
                            line: loc.start,
                            column: loc.end,
                        })?;
                    args.push(FullArgument {
                        name: arg_name.to_string(),
                        arg_type,
                    });
                }
                _ => {
                    return Err(LexerError {
                        input: input_str,
                        message: "Expected argument type after colon".to_string(),
                        line: loc.start,
                        column: loc.end,
                    });
                }
            }
        }
        _ => {
            return Err(LexerError {
                input: input_str,
                message: "Expected colon after argument name".to_string(),

                line: 0,
                column: 0,
            });
        }
    }
    Ok(args)
}

fn lex_macro_arguments(
    name: String,
    input: &str,
    lexer: &mut logos::SpannedIter<TokenKind>,
) -> Result<Vec<TokenKind>, LexerError> {
    let input_str = input.to_string();
    let mut tokens = Vec::new();
    let mut args = Vec::new();
    loop {
        // look through macro arguments
        let (val, _) = match lexer.next() {
            Some((v, l)) => (v, l),
            None => panic!(),
        };
        match val {
            Ok(TokenKind::Tab) | Ok(TokenKind::Whitespace) | Ok(TokenKind::Comma) => {
                continue;
            }
            Ok(TokenKind::Ident(arg_name)) => {
                match lex_single_macro_argument(arg_name, input, lexer) {
                    Ok(v) => args.extend(v),
                    Err(e) => panic!("{e}"),
                }
            }
            Ok(TokenKind::RightParen) => break,
            _ => {
                return Err(LexerError {
                    input: input_str,
                    message: "Invalid macro argument syntax".to_string(),

                    line: 0,
                    column: 0,
                });
            }
        }
    }
    let value = if let Some(v) = lexer.next() {
        v.0
    } else {
        panic!();
    };
    match value {
        Ok(TokenKind::LeftBrace) => {
            let mut brace_count = 1;
            let mut macro_tokens = Vec::new();

            for (tok, span) in lexer.by_ref() {
                match tok {
                    Ok(TokenKind::LeftBrace) => brace_count += 1,
                    Ok(TokenKind::RightBrace) => {
                        brace_count -= 1;
                        if brace_count == 0 {
                            break;
                        }
                    }
                    Ok(t) => macro_tokens.push(t),
                    _ => {
                        return Err(LexerError {
                            input: input_str,
                            message: "Invalid token in macro body".to_string(),

                            line: span.start,
                            column: span.end,
                        });
                    }
                }
            }
            tokens.push(TokenKind::Macro(MacroContent {
                name,
                args,
                tokens: macro_tokens,
            }));
        }
        _ => {
            return Err(LexerError {
                input: input_str,
                message: "Expected open brace to start macro body".to_string(),

                line: 0,
                column: 0,
            });
        }
    }
    Ok(tokens)
}

fn lex_macro(
    input: &str,
    lexer: &mut logos::SpannedIter<TokenKind>,
) -> Result<Vec<TokenKind>, LexerError> {
    // O.o macro spotted
    let input_str = input.to_string();
    let mut tokens = Vec::new();

    let name = if let Ok(TokenKind::Ident(v)) = lexer.next().unwrap().0 {
        v
    } else {
        return Err(LexerError {
            input: input_str,
            message: "expected ident after macro decl".to_string(),
            line: 0,
            column: 0,
        });
    };
    // start collecting goodies in the macro :3
    let (val, loc) = match lexer.next() {
        Some((v, l)) => (v, l),
        None => panic!(),
    };
    match val {
        Ok(TokenKind::LeftParen) => {
            // O.o look, macro arguments!
            match lex_macro_arguments(name, input, lexer) {
                Ok(v) => tokens.extend(v),
                Err(e) => panic!("{e}"),
            }
        }
        _ => {
            return Err(LexerError {
                input: input_str,
                message: "Expected open paren after macro name".to_string(),

                line: loc.start,
                column: loc.end,
            });
        }
    }
    Ok(tokens)
}

pub fn lex(input: &str) -> Result<Vec<TokenKind>, LexerError> {
    let input_str = input.to_string();
    let mut lexer = TokenKind::lexer(input).spanned();
    let mut tokens = Vec::new();

    while let Some((token, span)) = lexer.next() {
        // begin token iteration here
        match token {
            Ok(TokenKind::Whitespace) | Ok(TokenKind::Tab) => {}
            Ok(TokenKind::MacroDef(_)) => match lex_macro(input, &mut lexer) {
                Ok(v) => tokens.extend(v),
                Err(e) => return Err(e),
            },
            Ok(t) => {
                tokens.push(t);
                //column += lexer.slice().len();
            }
            Err(()) => {
                return Err(LexerError {
                    input: input_str,
                    message: "Unexpected token".to_string(),

                    line: span.start,
                    column: span.end,
                });
            }
        }
    }

    Ok(tokens)
}
