use basm2::*;
use colored::*;

fn main() {
    let input_string = r#"
    @include "my.asm"
    mov r0, 'a'
label: macro_rules! silly ( arg1: reg, arg2: imm, arg3: reg, arg4: mem) {
    mov %arg1, %arg2
    lea %arg2, %arg4
    .asciiz "Yap!\y"
}
    const memloc = 0xff
    silly!(r0, 3, r4, [(memloc + 3)])
    lea r0, [(memloc + 3)]
add r0, (((( ( 6 * 3 ) + (3 + 3) * 5) & ( 6 * 3 ) + (3 + 3) * 5) * 2 + (3 * 4 + 2) & 33) + (( ( 6 * 3 ) + (3 + 3) * 5) & ( 6 * 3 ) + (3 + 3) * 5) * 2 + (3 * 4 + 2) & 33))
    const c = 23
"#;

    let file = "input.asm";
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

    process_includes(&mut toks, input_string, &mut error_count);
    process_macros(&mut toks, input_string, &mut error_count);

    if error_count > 0 {
        print_errors(error_count);
        std::process::exit(1);
    }

    if CONFIG.verbose {
        print_msg!("COMPLETE TOKENS");
        for (_, f, _) in &toks {
            println!("{f}");
        }
    }
}

fn print_errors(error_count: i32) {
    let msg = if error_count == 1 {
        "error generated "
    } else {
        "errors generated"
    };
    println!(
        "compilation unsuccessful\n{} {}.",
        error_count.to_string().bright_red(),
        msg,
    );
}
