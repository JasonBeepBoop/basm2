use crate::*;
use std::iter::Peekable;
type Evalex<'a> = Peekable<logos::SpannedIter<'a, tokens::TokenKind>>;
pub fn evaluate_expression(input: String, token_iter: &mut Evalex) -> Result<i64, ParserError> {
    let mut result = 0;
    while let Some((token, l)) = token_iter.peek() {
        match token {
            Ok(TokenKind::RightParen) => {
                break;
            }
            Ok(TokenKind::IntLit(num)) => {
                result = *num;
                token_iter.next();
            }
            Ok(TokenKind::Plus) => {
                token_iter.next();
                if let Some((Ok(TokenKind::IntLit(num)), _)) = token_iter.peek() {
                    result += num;
                    token_iter.next();
                }
            }
            Ok(TokenKind::PlusPlus) => {
                token_iter.next();
                result += 1;
            }
            Ok(TokenKind::Star) => {
                token_iter.next();
                if let Some((Ok(TokenKind::IntLit(num)), _)) = token_iter.peek() {
                    result *= num;
                    token_iter.next();
                }
            }
            Ok(TokenKind::GreaterGreater) => {
                token_iter.next();
                if let Some((Ok(TokenKind::IntLit(num)), _)) = token_iter.peek() {
                    result >>= num;
                    token_iter.next();
                }
            }
            Ok(TokenKind::LessLess) => {
                token_iter.next();
                if let Some((Ok(TokenKind::IntLit(num)), _)) = token_iter.peek() {
                    result <<= num;
                    token_iter.next();
                }
            }
            Ok(TokenKind::Minus) => {
                token_iter.next();
                if let Some((Ok(TokenKind::IntLit(num)), _)) = token_iter.peek() {
                    result -= num;
                    token_iter.next();
                }
            }
            Ok(TokenKind::MinusMinus) => {
                token_iter.next();
                result -= 1;
            }
            _ => {
                return Err(ParserError {
                    input: input.to_string(),
                    message: "the math expression appears invalid".to_string(),
                    start_pos: l.start,
                    last_pos: l.end,
                });
            }
        }
    }
    Ok(result)
}
