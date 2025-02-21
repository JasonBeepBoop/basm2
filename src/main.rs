use basm2::*;

fn main() {
    let input_string = r#"

const     v = (4 * 3)
label: macro_rules! fanf ( arg1 : reg, arg2 : imm, arg3 : mem, arg4 : ireg, arg5 : label ) { 
    mov %arg1, %arg2 ; comment
    mov r0, &[0x0]
    label_again: .asciiz "My text"
    .word 'm'
    label: nand r4, (2 * 2)
    %arg5:
}
    mov r0, (v + 3)

    push (3 << 1)

    add r0, ( 2 & ( 6 * 3 ) + (3 + 3) * 5)
    add r0, (3 & 4)
    hlt

"#;
    let input_string_2 = r#"

const v = (4 * 3)
label: macro_rules! fanf ( arg1 : reg, arg2 : imm, arg3 : mem, arg4 : ireg, arg5 : label ) { 
    mov r0, (v + 2)
}

"#;
    println!("{input_string_2}");
    let my_macaroni = MacroContent {
        file: String::from("aw"),
        name: String::from("ka"),
        args: vec![(FullArgument {
            name: String::from("frank"),
            arg_type: ArgumentType::Reg,
        }, 0..0)],
        tokens: Vec::new(),
    };
    my_macaroni.is_valid(vec![(TokenKind::Register(3), 0..0)]); // this is working
    
    let mut parser = match Parser::new(String::from("input.asm"), input_string_2) {
        Ok(v) => v,
        Err(e) => {
            for er in e {
                println!("{er}");
            }
            std::process::exit(1);
        }
    };
    match parser.parse() {
        Ok(tokens) => {
            //println!("{#:?}", serde_json::to_string_pretty(&tokens).unwrap());
            for (element, _) in tokens {
                println!("{}", element);
            }
        }
        Err(e) => {
            for error in e {
                println!("{error}");
            }
            std::process::exit(1);
        }
    }
}
