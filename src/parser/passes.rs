use crate::eval::evaluate_expression;
use crate::*;
type PassOne = Result<Vec<(Result<TokenKind, ()>, std::ops::Range<usize>)>, Vec<ParserError>>;
impl<'a> Parser<'a> {
    pub fn first_pass(
        file: String,
        input: String,
        lexer: logos::SpannedIter<'a, TokenKind>,
    ) -> PassOne {
        let mut tokens = Vec::new();
        let mut lexer = lexer.peekable();
        let mut errors = Vec::new();
        let mut const_names = Vec::new();
        let mut prev_was_const = false;
        let mut saw_amp = false;
        while let Some((token, span)) = lexer.next() {
            match token {
                Ok(TokenKind::Amp) => saw_amp = true,
                Ok(TokenKind::MacroDef(m)) => {
                    saw_amp = false;
                    // this here to make sure leftparen doesn't
                    // accidentally start reading a macro
                    prev_was_const = false;
                    tokens.push((Ok(TokenKind::MacroDef(m)), span));
                    'mdl: loop {
                        match lexer.next() {
                            Some((Ok(TokenKind::LeftBrace), l)) => {
                                tokens.push((Ok(TokenKind::LeftBrace), l));
                                break 'mdl;
                            }
                            Some((Ok(TokenKind::Ident(ident)), span)) => {
                                if let Some((Ok(TokenKind::Colon), _)) = lexer.peek() {
                                    let (_, _) = lexer.next().unwrap();
                                    tokens.push((Ok(TokenKind::Label(ident)), span));
                                } else {
                                    tokens.push((Ok(TokenKind::Ident(ident)), span));
                                }
                            }
                            Some(v) => tokens.push(v),
                            _ => break 'mdl,
                        }
                    }
                }
                Ok(TokenKind::Constant(name)) => {
                    saw_amp = false;
                    const_names.push(name);
                    prev_was_const = true;
                    if let Some((Ok(TokenKind::Equal), _)) = lexer.peek() {
                        lexer.next();
                    } else {
                        errors.push(ParserError {
                            file: file.to_string(),
                            help: None,
                            input: input.to_string(),
                            message: String::from(
                                "constant requires equal sign to denote assignment",
                            ),
                            start_pos: span.start,
                            last_pos: span.end,
                        });
                    }
                }
                Ok(TokenKind::Ident(ident)) => {
                    saw_amp = false;
                    if let Some((Ok(TokenKind::Colon), _)) = lexer.peek() {
                        let (_, _) = lexer.next().unwrap();
                        tokens.push((Ok(TokenKind::Label(ident)), span));
                    } else {
                        tokens.push((Ok(TokenKind::Ident(ident)), span));
                    }
                    prev_was_const = false;
                }
                Ok(TokenKind::IntLit(v)) => {
                    saw_amp = false;
                    if prev_was_const {
                        if let Some(n) = const_names.pop() {
                            let mut vmap = V_MAP.lock().unwrap();
                            vmap.insert(n, (file.to_string(), span, v));
                            std::mem::drop(vmap);
                        } else {
                            errors.push(ParserError {
                                file: file.to_string(),
                                help: None,
                                input: input.to_string(),
                                message: String::from(
                                    "could not find associated constant for literal",
                                ),
                                start_pos: span.start,
                                last_pos: span.end,
                            });
                        }
                    } else {
                        tokens.push((Ok(TokenKind::IntLit(v)), span));
                    }
                }
                Ok(TokenKind::LeftBracket) => {
                    // memory addresses are also not instructions
                    let mut addr_toks = Vec::new();
                    'mdl: loop {
                        match lexer.next() {
                            // let's try to do math in it
                            Some((Ok(TokenKind::LeftParen), span)) => {
                                match parse_expression_after_left_paren(
                                    &file,
                                    input.to_string(),
                                    &mut lexer,
                                ) {
                                    Ok(Some((value, new_span))) => {
                                        addr_toks.push((TokenKind::IntLit(value), new_span));
                                    }
                                    Ok(None) => {
                                        addr_toks.push((TokenKind::LeftParen, span));
                                        break 'mdl;
                                    }
                                    Err(e) => {
                                        errors.push(e);
                                    }
                                }
                            }
                            Some((Ok(TokenKind::RightBracket), _)) => {
                                break 'mdl;
                            }
                            Some((Ok(TokenKind::Ident(ident)), span)) => {
                                if let Some((Ok(TokenKind::Colon), _)) = lexer.peek() {
                                    let (_, _) = lexer.next().unwrap();
                                    addr_toks.push(((TokenKind::Label(ident)), span));
                                } else {
                                    addr_toks.push(((TokenKind::Ident(ident)), span));
                                }
                            }
                            Some((Ok(TokenKind::RightParen), _)) => (),
                            Some((Ok(v), span)) => addr_toks.push((v, span)),
                            _ => break 'mdl,
                        }
                    }
                    tokens.push((
                        Ok(TokenKind::Mem(MemAddr {
                            indirect: saw_amp,
                            content: addr_toks,
                        })),
                        span,
                    ));
                    saw_amp = false;
                }
                Ok(TokenKind::LeftParen) => 'lpn: {
                    saw_amp = false;
                    match parse_expression_after_left_paren(&file, input.to_string(), &mut lexer) {
                        Ok(Some((value, new_span))) => {
                            if prev_was_const {
                                if let Some(n) = const_names.pop() {
                                    let mut vmap = V_MAP.lock().unwrap();
                                    vmap.insert(n, (file.to_string(), new_span, value));
                                    std::mem::drop(vmap);
                                } else {
                                    errors.push(ParserError {
                                        file: file.to_string(),
                                        help: None,
                                        input: input.to_string(),
                                        message: String::from(
                                            "could not find associated constant name",
                                        ),
                                        start_pos: new_span.start,
                                        last_pos: new_span.end,
                                    });
                                }
                            } else {
                                tokens.push((Ok(TokenKind::IntLit(value)), new_span));
                            }
                        }
                        Ok(None) => {
                            tokens.push((Ok(TokenKind::LeftParen), span));
                            break 'lpn;
                        }
                        Err(e) => {
                            errors.push(e);
                        }
                    }
                    if let Some((Ok(TokenKind::RightParen), _)) = lexer.peek() {
                        lexer.next();
                    }
                }
                _ => {
                    saw_amp = false;
                    tokens.push((token, span));
                }
            }
        }

        if !errors.is_empty() {
            return Err(errors);
        }
        Ok(tokens)
    }
    pub fn second_pass(
        &mut self,
        tokens: Vec<(Result<TokenKind, ()>, std::ops::Range<usize>)>,
    ) -> Vec<(Result<TokenKind, ()>, std::ops::Range<usize>)> {
        let mut new_tokens = Vec::new();
        let mut token_iter = tokens.into_iter().peekable();
        while let Some((token, span)) = token_iter.next() {
            match token {
                Ok(TokenKind::MacroCall(m)) => {
                    // macro calls are not instructions
                    new_tokens.push((Ok(TokenKind::MacroCall(m)), span));
                    'mdl: loop {
                        match token_iter.next() {
                            Some((Ok(TokenKind::Newline), l)) => {
                                new_tokens.push((Ok(TokenKind::Newline), l));
                                break 'mdl;
                            }
                            Some((Ok(TokenKind::Ident(ident)), span)) => {
                                if let Some((Ok(TokenKind::Colon), _)) = token_iter.peek() {
                                    let (_, _) = token_iter.next().unwrap();
                                    new_tokens.push((Ok(TokenKind::Label(ident)), span));
                                } else {
                                    new_tokens.push((Ok(TokenKind::Ident(ident)), span));
                                }
                            }
                            Some(v) => new_tokens.push(v),
                            _ => break 'mdl,
                        }
                    }
                }

                Ok(TokenKind::Ident(name)) => {
                    let mut has_colon = false;
                    let mut peek_iter = token_iter.clone();
                    while let Some((peek_token, _)) = peek_iter.peek() {
                        match peek_token {
                            Ok(TokenKind::Newline) => break,
                            Ok(TokenKind::Label(_)) => {
                                has_colon = true;
                                break;
                            }
                            Ok(TokenKind::LeftBrace) | Ok(TokenKind::StringLit(_)) => {
                                has_colon = true;
                                break;
                            }
                            _ => {
                                peek_iter.next();
                            }
                        }
                    }

                    if has_colon {
                        new_tokens.push((Ok(TokenKind::Ident(name)), span));
                    } else {
                        let mut args = Vec::new();
                        while let Some((token, loc)) = token_iter.peek() {
                            match token {
                                Ok(TokenKind::Comma) => {
                                    token_iter.next();
                                }
                                Ok(TokenKind::Newline) => {
                                    break;
                                }
                                Ok(t) => {
                                    if let Some(v) = self.parse_argument(t.clone()) {
                                        args.push((v, loc.clone()));
                                    }
                                    token_iter.next();
                                }
                                _ => {
                                    token_iter.next();
                                }
                            }
                        }
                        new_tokens.push((
                            Ok(TokenKind::Instruction(InstructionData { name, args })),
                            span,
                        ));
                    }
                }
                _ => {
                    new_tokens.push((token, span));
                }
            }
        }
        new_tokens
    }
}

fn parse_expression_after_left_paren(
    file: &str,
    input: String,
    lexer: &mut std::iter::Peekable<logos::SpannedIter<'_, TokenKind>>,
) -> Result<Option<(i64, logos::Span)>, ParserError> {
    let mut peek_iter = lexer.clone();
    while let Some((peek_token, _)) = peek_iter.peek() {
        match peek_token {
            Ok(TokenKind::Newline) => break,
            Ok(TokenKind::Colon) | Ok(TokenKind::LeftBrace) => {
                return Ok(None);
            }
            _ => {
                peek_iter.next();
            }
        }
    }

    let next_token = lexer.peek().cloned();
    match next_token {
        Some((Ok(_), span)) => {
            let value = evaluate_expression(&file.to_string(), input.to_string(), lexer)?;
            return Ok(Some((value, span.clone())));
        }
        Some((Err(_), span)) => {
            return Err(ParserError {
                file: file.to_string(),
                help: None,
                input: input.to_string(),
                message: String::from("invalid token in expression"),
                start_pos: span.start,
                last_pos: span.end,
            });
        }
        None => {}
    }

    Err(ParserError {
        file: file.to_string(),
        help: None,
        input: input.to_string(),
        message: String::from("failed to parse expression after left paren"),
        start_pos: 0,
        last_pos: 0,
    })
}
