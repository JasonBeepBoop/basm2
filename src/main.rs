use basm2::*;

fn main() {
    let input_string = r#"

label: macro_rules! silly ( arg1: reg, arg2: imm, arg3: reg, arg4: mem) { 
    mov %arg1, %arg2
    lea %arg2, %arg4
}
    const memloc = 0xff
    lea r0, [(memloc + 3)]
    silly!(r3, 3, r2, [0xff])
add r0, (((( ( 6 * 3 ) + (3 + 3) * 5) & ( 6 * 3 ) + (3 + 3) * 5) * 2 + (3 * 4 + 2) & 33) + (( ( 6 * 3 ) + (3 + 3) * 5) & ( 6 * 3 ) + (3 + 3) * 5) * 2 + (3 * 4 + 2) & 33))
"#;
    let input_string_2 = r#"
const v = (4 * 3)
    mov r0, (v + 2)
    const a = 5
    const b = 3
    ;add r0, (((( ( 6 * 3 ) + (3 + 3) * 5) & ( 6 * 3 ) + (3 + 3) * 5) * 2 + (3 * 4 + 2) & 33) + (( ( 6 * 3 ) + (3 + 3) * 5) & ( 6 * 3 ) + (3 + 3) * 5) * 2 + (3 * 4 + 2) & 33))
    const c = ((a * a) + (b * b))
    mov r0, (c)
    macro_rules! fnaf ( arg1 : imm ) {
        mov r0, %arg1
    }
    [( 2 * (c + 3))]
"#;
    println!("{input_string}");
    let mut parser = match Parser::new(String::from("input.asm"), input_string) {
        Ok(v) => v,
        Err(e) => {
            for er in e {
                println!("{er}");
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
                println!("{error}");
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
    for (element, span) in toks {
        if let MacroCall(call) = element {
            in_call = true;
            mac_call_data = Vec::new();
            if let Some(v) = mac_map.get(&call) {
                curr_mac = Some(v);
            } else {
                println!("uwu :3 i cannot find this macro with name {call}");
                curr_mac = None;
            }
            continue;
        }
        if let RightParen = element {
            in_call = false;
            if let Some((_, m)) = curr_mac {
                match m.is_valid(input_string.to_string(), mac_call_data.clone()) {
                    Ok(v) => {
                        println!("expanded toks:");
                        for (tok, _) in v {
                            println!("{tok}");
                        }
                    }
                    Err(e) => {
                        for e in e {
                            println!("{e}");
                        }
                    }
                }
            }
            continue;
        }
        if in_call {
            mac_call_data.push((element.clone(), span.clone()));
        }
    }
}
