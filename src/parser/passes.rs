use crate::evaluate::evaluate_expression;
use crate::*;
impl<'a> Parser<'a> {
    pub fn first_pass(
        input: String,
        lexer: logos::SpannedIter<'a, TokenKind>,
    ) -> Vec<(Result<TokenKind, ()>, std::ops::Range<usize>)> {
        let mut tokens = Vec::new();
        let mut lexer = lexer.peekable();

        while let Some((token, span)) = lexer.next() {
            match token {
                Ok(TokenKind::Ident(ident)) => {
                    if let Some((Ok(TokenKind::Colon), _)) = lexer.peek() {
                        let (_, _) = lexer.next().unwrap();

                        tokens.push((Ok(TokenKind::Label(ident)), span));
                    } else {
                        tokens.push((Ok(TokenKind::Ident(ident)), span));
                    }
                }

                Ok(TokenKind::LeftParen) => 'lpn: {
                    let mut peek_iter = lexer.clone();
                    while let Some((peek_token, _)) = peek_iter.peek() {
                        match peek_token {
                            Ok(TokenKind::Newline) => break,
                            Ok(TokenKind::Colon) | Ok(TokenKind::LeftBrace) => {
                                tokens.push((Ok(TokenKind::LeftParen), span));
                                break 'lpn;
                            }
                            _ => {
                                peek_iter.next();
                            }
                        }
                    }

                    while let Some((token, _)) = lexer.peek() {
                        match token {
                            Ok(TokenKind::Comma) => {
                                lexer.next();
                            }
                            Ok(TokenKind::Newline) => {
                                break 'lpn;
                            }
                            Ok(_) => {
                                match evaluate_expression(input.to_string(), &mut lexer) {
                                    Ok(v) => tokens.push((Ok(TokenKind::IntLit(v)), span.clone())),
                                    Err(e) => {
                                        println!("{e:?}");
                                        panic!(); // :3 nothing will go wrong
                                    }
                                }
                                lexer.next();
                            }
                            _ => {
                                lexer.next();
                            }
                        }
                    }
                }

                _ => {
                    tokens.push((token, span));
                }
            }
        }

        tokens
    }

    pub fn second_pass(
        &mut self,
        tokens: Vec<(Result<TokenKind, ()>, std::ops::Range<usize>)>,
    ) -> Vec<(Result<TokenKind, ()>, std::ops::Range<usize>)> {
        let mut new_tokens = Vec::new();
        let mut token_iter = tokens.into_iter().peekable();

        while let Some((token, span)) = token_iter.next() {
            match token {
                Ok(TokenKind::Ident(name)) => {
                    let mut has_colon = false;
                    let mut peek_iter = token_iter.clone();
                    while let Some((peek_token, _)) = peek_iter.peek() {
                        match peek_token {
                            Ok(TokenKind::Newline) => break,
                            Ok(TokenKind::Colon)
                            | Ok(TokenKind::LeftBrace)
                            | Ok(TokenKind::StringLit(_)) => {
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
                                    args.push((self.parse_argument(t.clone()), loc.clone()));
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
