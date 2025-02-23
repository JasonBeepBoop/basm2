use basm2::*;
fn main() {
    let input_string = read_file(&CONFIG.source);

    let file = &CONFIG.source;
    let mut error_count = 0;

    if CONFIG.verbose {
        print_msg!("RAW INPUT");
        println!("{input_string}");
    }

    let mut parser = match create_parser(file, &input_string, &mut error_count) {
        Some(parser) => parser,
        None => std::process::exit(1),
    };
    print_errc!(error_count);

    let mut toks = match parse_tokens(&mut parser, &input_string, &mut error_count) {
        Some(tokens) => tokens,
        None => std::process::exit(1),
    };
    print_errc!(error_count);

    process_includes(&mut toks, &mut error_count);

    process_macros(&mut toks, &mut error_count);

    process_start(&mut toks, &mut error_count);

    if CONFIG.verbose {
        print_msg!("COMPLETE TOKENS");
        for (_, f, _) in &toks {
            println!("{f}");
        }
    }
    print_errc!(error_count);
    let l_map = LABEL_MAP.lock().unwrap();
    println!("{l_map:?}");
    print_errc!(error_count);
}
