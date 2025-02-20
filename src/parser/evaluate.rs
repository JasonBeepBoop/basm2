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
            Ok(TokenKind::Star) => {
                token_iter.next();
                if let Some((Ok(TokenKind::IntLit(num)), _)) = token_iter.peek() {
                    result *= num;
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
            _ => {
                return Err(ParserError {
                    input: input.to_string(),
                    message: "Invalid expression".to_string(),
                    start_pos: l.start,
                    last_pos: l.end,
                });
            }
        }
    }
    Ok(result)
}
