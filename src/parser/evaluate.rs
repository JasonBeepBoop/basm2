use crate::*;
use std::iter::Peekable;
use std::vec::IntoIter;
type Evalex = Peekable<IntoIter<(Result<TokenKind, ()>, std::ops::Range<usize>)>>;
impl Parser<'_> {
    pub fn evaluate_expression(&mut self, token_iter: &mut Evalex) -> i64 {
        let mut result = 0;
        while let Some((token, l)) = token_iter.peek() {
            match token {
                Ok(TokenKind::RightParen) => {
                    token_iter.next();
                    break;
                }
                Ok(TokenKind::IntLit(num)) => {
                    result = *num;
                    token_iter.next();
                }
                Ok(TokenKind::Plus) => {
                    token_iter.next();
                    if let Some((Ok(TokenKind::IntLit(num)), _)) = token_iter.peek() {
                        result += *num;
                        token_iter.next();
                    }
                }
                Ok(TokenKind::Minus) => {
                    token_iter.next();
                    if let Some((Ok(TokenKind::IntLit(num)), _)) = token_iter.peek() {
                        result -= *num;
                        token_iter.next();
                    }
                }
                _ => {
                    self.errors.push(ParserError {
                        input: self.input.to_string(),
                        message: "Invalid expression".to_string(),
                        start_pos: l.start,
                        last_pos: l.end,
                    });
                    break;
                }
            }
        }
        result
    }
}
