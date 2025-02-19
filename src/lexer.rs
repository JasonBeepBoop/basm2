use crate::*;
use logos::Logos;
use std::fmt;

#[derive(Debug)]
pub struct LexerError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Error at line {}:{} - {}",
            self.line, self.column, self.message
        )
    }
}
pub fn lex(input: &str) -> Result<Vec<TokenKind>, LexerError> {
    let mut lexer = TokenKind::lexer(input).peekable();
    let mut tokens = Vec::new();
    let mut line = 1;
    let mut column = 1;

    while let Some(token) = lexer.next() {
        // begin token iteration here
        match token {
            Ok(TokenKind::Newline) => {
                line += 1;
                column = 1;
                tokens.push(TokenKind::Newline);
            }
            Ok(TokenKind::Whitespace) | Ok(TokenKind::Tab) => {}
            Ok(TokenKind::MacroDef(macro_def)) => {
                // O.o macro spotted
                let name = macro_def
                    .split_whitespace()
                    .nth(1)
                    .ok_or_else(|| LexerError {
                        message: "Macro definition should have a name".to_string(),
                        line,
                        column,
                    })?
                    .to_string();
                // start collecting goodies in the macro :3
                match lexer.next() {
                    Some(Ok(TokenKind::LeftParen)) => {
                        // O.o look, macro arguments!
                        let mut args = Vec::new();
                        loop {
                            // look through macro arguments
                            match lexer.next() {
                                Some(Ok(TokenKind::Tab))
                                | Some(Ok(TokenKind::Whitespace))
                                | Some(Ok(TokenKind::Comma)) => {
                                    continue;
                                }
                                Some(Ok(TokenKind::Ident(arg_name))) => {
                                    // teehee,
                                    // found an argument
                                    match lexer.next() {
                                        Some(Ok(TokenKind::Tab))
                                        | Some(Ok(TokenKind::Whitespace)) => {
                                            continue;
                                        }
                                        Some(Ok(TokenKind::Colon)) => {
                                            match lexer.next() {
                                                Some(Ok(TokenKind::Tab))
                                                | Some(Ok(TokenKind::Whitespace)) => {
                                                    continue;
                                                }
                                                Some(Ok(TokenKind::Ident(arg_type_str))) => {
                                                    let arg_type =
                                                        ArgumentType::from_str(&arg_type_str)
                                                            .ok_or_else(|| LexerError {
                                                                message: format!(
                                                                    "Invalid argument type: {}",
                                                                    arg_type_str
                                                                ),
                                                                line,
                                                                column,
                                                            })?;
                                                    args.push(FullArgument {
                                                        name: arg_name.to_string(),
                                                        arg_type,
                                                    });
                                                }
                                                _ => {
                                                    return Err(LexerError {
                                                        message:
                                                            "Expected argument type after colon"
                                                                .to_string(),
                                                        line,
                                                        column,
                                                    });
                                                }
                                            }
                                        }
                                        _ => {
                                            return Err(LexerError {
                                                message: "Expected colon after argument name"
                                                    .to_string(),
                                                line,
                                                column,
                                            });
                                        }
                                    }
                                }
                                Some(Ok(TokenKind::RightParen)) => break,
                                _ => {
                                    return Err(LexerError {
                                        message: "Invalid macro argument syntax".to_string(),
                                        line,
                                        column,
                                    });
                                }
                            }
                        }
                        match lexer.next() {
                            Some(Ok(TokenKind::Tab)) | Some(Ok(TokenKind::Whitespace)) => {
                                continue;
                            }
                            Some(Ok(TokenKind::LeftBrace)) => {
                                let mut brace_count = 1;
                                let mut macro_tokens = Vec::new();

                                for tok in lexer.by_ref() {
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
                                                message: "Invalid token in macro body".to_string(),
                                                line,
                                                column,
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
                                    message: "Expected open brace to start macro body".to_string(),
                                    line,
                                    column,
                                });
                            }
                        }
                    }
                    _ => {
                        return Err(LexerError {
                            message: "Expected open paren after macro name".to_string(),
                            line,
                            column,
                        });
                    }
                }
            }
            Ok(t) => {
                tokens.push(t);
                //column += lexer.slice().len();
            }
            Err(()) => {
                return Err(LexerError {
                    message: "Unexpected token".to_string(),
                    line,
                    column,
                });
            }
        }
    }

    Ok(tokens)
}