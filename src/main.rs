use basm2::*;
use colored::*;
use std::collections::HashMap;
fn main() {
    let input_string = r#"

label: macro_rules! silly ( arg1: reg, arg2: imm, arg3: reg, arg4: mem) { 
    mov %arg1, %arg2
    lea %arg3, %arg4
}
    const memloc = 0xff
    lea r0, [(memloc + 3)]
    silly!(r3, 3, r2, [0xff])
add r0, (((( ( 6 * 3 ) + (3 + 3) * 5) & ( 6 * 3 ) + (3 + 3) * 5) * 2 + (3 * 4 + 2) & 33) + (( ( 6 * 3 ) + (3 + 3) * 5) & ( 6 * 3 ) + (3 + 3) * 5) * 2 + (3 * 4 + 2) & 33))
"#;
    println!("{input_string}");
    let file = "input.asm";
    let mut parser = match Parser::new(String::from(file), input_string) {
        Ok(v) => v,
        Err(e) => {
            for er in e {
                println!("{er}\n");
            }
            std::process::exit(1);
        }
    };
    let mut toks = match parser.parse() {
        Ok(tokens) => {
            //println!("{#:?}", serde_json::to_string_pretty(&tokens).unwrap());
            /*for (element, _) in &tokens {
                println!("{}", element);
            }*/
            tokens
        }
        Err(e) => {
            for error in e {
                println!("{error}\n");
            }
            std::process::exit(1);
        }
    };
    use crate::TokenKind::*;
    let mut mac_locs = Vec::new();
    for (index, (element, _)) in toks.iter().enumerate() {
        if let Macro(data) = element {
            let mut mac_map = MACRO_MAP.lock().unwrap();
            mac_map.insert(
                data.name.0.to_string(),
                (data.file.to_string(), data.clone()),
            );
            mac_locs.push(index);
        }
    }
    for element in mac_locs {
        toks.remove(element);
    }
    let mut mac_call_data = Vec::new();
    let mut in_call = false;
    let mut curr_mac = None;
    let mac_map = MACRO_MAP.lock().unwrap();
    let mut expanded_loc_map: HashMap<usize, Vec<(TokenKind, std::ops::Range<usize>)>> =
        HashMap::new();
    let mut expanded_indices = Vec::new();
    // collecting macro arguments upon macro calls
    let mut counter = 0;
    for (element, span) in &toks {
        counter += 1;
        if let MacroCall(call) = element {
            counter -= 1;
            in_call = true;
            mac_call_data = Vec::new();
            if let Some(v) = mac_map.get(call) {
                curr_mac = Some(v);
            } else {
                let problem = ParserError {
                    file: file.to_string(),
                    help: None,
                    input: input_string.to_string(),
                    message: format!(
                        "{}: with name `{}`",
                        "cannot find macro".bold(),
                        call.bold()
                    ),
                    start_pos: span.start,
                    last_pos: span.end,
                };
                println!("{problem}\n");
                curr_mac = None;
            }
            continue;
        }
        if let RightParen = element {
            counter -= 1;
            in_call = false;
            if let Some((_, m)) = curr_mac {
                match m.is_valid(input_string.to_string(), mac_call_data.clone()) {
                    Ok(v) => {
                        expanded_loc_map.insert(counter, v.clone());
                        expanded_indices.push(counter);
                        println!("expanded toks:");
                        for (tok, _) in v {
                            println!("{tok}");
                        }
                    }
                    Err(e) => {
                        for e in e {
                            println!("{e}\n");
                        }
                    }
                }
            }
            continue;
        }
        if in_call {
            counter -= 1;
            mac_call_data.push((element.clone(), span.clone()));
        }
    }
    let size = toks.len();
    for i in 0..size {
        if expanded_indices.contains(&i) {
            let expanded = expanded_loc_map.get(&i).unwrap(); // this never fails as all pairs match
            for element in expanded.into_iter().rev() {
                toks.insert(i, element.clone());
            }
        }
    }
    let mut new_tokens = Vec::new();
    let mut tokerator = toks.clone().into_iter();
    while let Some((v, s)) = tokerator.next() {
        match v {
            MacroCall(_) => {
                while let Some((val, span)) = tokerator.next() {
                    match val {
                        RightParen => break,
                        _ => (),
                    }
                }
            }
            _ => new_tokens.push((v, s)),
        }
    }
    toks = new_tokens;
    println!("{toks:#?}");
}
