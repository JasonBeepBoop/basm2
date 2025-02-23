use basm2::*;
use colored::*;

fn main() {
    let input_string = r#"
    @include "my.asm"
    mov r0, 'a'
label: macro_rules! silly ( arg1: reg, arg2: imm, arg3: reg, arg4: mem) {
    mov %arg1, %arg2
    lea %arg3, %arg4
    .asciiz "Yap!\y"
}
    const memloc = 0xff
    silly!(r0, 3, r4, [(memloc + 3)])
    lea r0, [(memloc + 3)]
add r0, (((( ( 6 * 3 ) + (3 + 3) * 5) & ( 6 * 3 ) + (3 + 3) * 5) * 2 + (3 * 4 + 2) & 33) + (( ( 6 * 3 ) + (3 + 3) * 5) & ( 6 * 3 ) + (3 + 3) * 5) * 2 + (3 * 4 + 2) & 33))
    const c = 23
"#;

    let file = "test.asm";
    let mut error_count = 0;

    if CONFIG.verbose {
        print_msg!("RAW INPUT");
        println!("{input_string}");
    }

    let mut parser = match create_parser(file, input_string, &mut error_count) {
        Some(parser) => parser,
        None => std::process::exit(1),
    };

    let mut toks = match parse_tokens(&mut parser, input_string, &mut error_count) {
        Some(tokens) => tokens,
        None => std::process::exit(1),
    };

    process_includes(&mut toks, &mut error_count);
    process_macros(&mut toks, &mut error_count);

    if CONFIG.verbose {
        print_msg!("COMPLETE TOKENS");
        for (_, f, _) in &toks {
            println!("{f}");
        }
    }
    use crate::TokenKind::*;
    let mut toks_iter = toks.clone().into_iter().peekable();
    let mut start_addr = 100;
    let mut seen_start = false;
    while let Some((fname, tok, span)) = toks_iter.next() {
        match tok {
            Directive(data) => match data.as_str() {
                "start" => {
                    if seen_start {
                        handle_include_error(
                            &fname,
                            &span,
                            &mut error_count,
                            ".start directive can only be declared once",
                            None,
                        );
                        break;
                    }
                    if let Some((f, TokenKind::IntLit(val), s)) = toks_iter.peek() {
                        start_addr = *val;
                        seen_start = true;
                    } else {
                        handle_include_error(
                            &fname,
                            &span,
                            &mut error_count,
                            ".start directive must be succeeded by integer literal",
                            None,
                        );

                        break;
                    }
                }
                _ => (),
            },
            _ => (),
        }
    }
    let mut toks_iter = toks.clone().into_iter().peekable();
    let mut l_map = LABEL_MAP.lock().unwrap();
    let mut loc_counter = start_addr;
    while let Some((fname, tok, span)) = toks_iter.next() {
        match tok {
            Label(name) => {
                l_map.insert(
                    name,
                    (fname.to_string(), span.clone(), loc_counter as usize),
                );
            }
            Directive(data) => match data.as_str() {
                "pad" => {
                    if let Some((_, TokenKind::IntLit(v), _)) = toks_iter.peek() {
                        loc_counter += v;
                        toks_iter.next();
                    } else {
                        handle_include_error(
                            &fname,
                            &span,
                            &mut error_count,
                            ".pad directive must be succeeded by literal",
                            None,
                        );
                        break;
                    }
                }
                "word" => {
                    if toks_iter
                        .peek()
                        .is_some_and(|v| matches!(v.1, TokenKind::Ident(_) | TokenKind::IntLit(_)))
                    {
                        loc_counter += 1;
                        toks_iter.next();
                    } else {
                        handle_include_error(
                            &fname,
                            &span,
                            &mut error_count,
                            ".word directive must be succeeded by literal",
                            None,
                        );
                        break;
                    }
                }
                "asciiz" => {
                    if let Some((f, TokenKind::StringLit(val), _)) = toks_iter.peek() {
                        loc_counter += val.len() as i64;
                        toks_iter.next();
                    } else {
                        handle_include_error(
                            &fname,
                            &span,
                            &mut error_count,
                            ".asciiz directive must be succeeded by string",
                            None,
                        );
                        break;
                    }
                }
                _ => {
                    handle_include_error(
                        &fname,
                        &span,
                        &mut error_count,
                        &format!("unrecognized directive {data}"),
                        None,
                    );
                    break;
                }
            },
            Instruction(_) => loc_counter += 1,
            Newline => (),
            _ => {
                handle_include_error(
                    &fname,
                    &span,
                    &mut error_count,
                    &format!("unrecognized {tok}"),
                    None,
                );
                break;
            }
        }
    }
    println!("{l_map:?}");
    if error_count > 0 {
        print_errors(error_count);
        std::process::exit(1);
    }
}

fn print_errors(error_count: i32) {
    let msg = if error_count == 1 {
        "error generated"
    } else {
        "errors generated"
    };
    println!(
        "compilation unsuccessful\n{} {}.",
        error_count.to_string().bright_red(),
        msg,
    );
}
