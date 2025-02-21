use crate::*;
use std::iter::Peekable;

type Evalex<'a> = Peekable<logos::SpannedIter<'a, tokens::TokenKind>>;

pub fn parse_expression(
    file: String,
    input: String,
    token_iter: &mut Evalex,
) -> Result<Expr, ParserError> {
    /*if let Ok(ref d) = v {
        let e = d.evaluate();
        println!("{e}\n{d}");
    }*/
    parse_bitwise(file, input, token_iter)
}

pub fn parse_primary(
    file: String,
    input: String,
    token_iter: &mut Evalex,
) -> Result<Expr, ParserError> {
    let mut last_loc = 0..0;

    if let Some((_, loc)) = token_iter.peek() {
        last_loc = loc.clone();
    }
    if let Some((token, l)) = token_iter.next() {
        last_loc = l.clone();

        match token {
            Ok(TokenKind::IntLit(num)) => Ok(Expr::Int(num)),
            Ok(TokenKind::LeftParen) => {
                let expr = parse_expression(file.to_string(), input.to_string(), token_iter)?;
                if let Some((Ok(TokenKind::RightParen), _)) = token_iter.next() {
                    Ok(expr)
                } else {
                    Err(ParserError {
                        file: file.to_string(),
                        help: None,
                        input: input.to_string(),
                        message: "unmatched parenthesis".to_string(),
                        start_pos: last_loc.start,
                        last_pos: last_loc.end,
                    })
                }
            }
            Ok(TokenKind::Ident(val)) => {
                let vmap = VARIABLE_MAP.lock().unwrap();
                if let Some((_, _, v)) = vmap.get(&val) {
                    let val = Ok(Expr::Int(*v));
                    std::mem::drop(vmap);
                    val
                } else {
                    Err(ParserError {
                        file: file.to_string(),
                        help: None,
                        input: input.to_string(),
                        message: format!("constant with name {val} not found"), 
                        start_pos: last_loc.start,
                        last_pos: last_loc.end,
                    })
                }
            }
            Ok(v) => Err(ParserError {
                file: file.to_string(),
                help: None,
                input: input.to_string(),
                message: format!("unexpected {v} in expression"),
                start_pos: last_loc.start,
                last_pos: last_loc.end,
            }),
            _ => Err(ParserError {
                file: file.to_string(),
                help: None,
                input: input.to_string(),
                message: String::from("reached an error while parsing expression\n      maybe the expression is invalid?"),
                start_pos: last_loc.start,
                last_pos: last_loc.end,
            }),
        }
    } else {
        Err(ParserError {
            file: file.to_string(),
            help: None,
            input: input.to_string(),
            message: "unexpected end of expression".to_string(),
            start_pos: last_loc.start,
            last_pos: last_loc.end,
        })
    }
}

pub fn evaluate_expression(
    file: &String,
    input: String,
    token_iter: &mut Evalex,
) -> Result<i64, ParserError> {
    let expr = parse_expression(file.to_string(), input, token_iter)?;
    println!("{}", expr.evaluate());
    println!("{expr}");
    Ok(expr.evaluate())
}
