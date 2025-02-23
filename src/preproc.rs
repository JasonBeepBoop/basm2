use crate::*;
use std::fs::File;
use std::io::Read;
pub fn process_includes(
    toks: &mut Vec<(String, TokenKind, std::ops::Range<usize>)>,
    error_count: &mut i32,
) {
    loop {
        let mut included_toks = Vec::new();
        let mut index = 0;
        let mut has_include = false;

        use crate::TokenKind::*;
        #[allow(clippy::explicit_counter_loop)]
        for (fname, element, loc) in toks.iter() {
            if let IncludeFile(new_file) = element {
                has_include = true;
                if *new_file == *fname {
                    handle_include_error(
                        fname,
                        loc,
                        error_count,
                        &format!("cannot include {fname} in itself"),
                        None,
                    );
                    println!("{toks:#?}");
                    break;
                }
                let contents = read_file(new_file);
                if let Some(mut parser) = create_parser(new_file, &contents, error_count) {
                    if let Some(tokens) = parse_tokens(&mut parser, &contents, error_count) {
                        for token in tokens.into_iter().rev() {
                            included_toks.insert(index, token);
                        }
                    }
                }
            } else {
                included_toks.push((fname.to_string(), element.clone(), loc.clone()));
            }
            index += 1;
        }

        *toks = included_toks;
        if !has_include {
            break;
        }
    }
}

pub fn read_file(file_path: &str) -> String {
    let mut file_data = match File::open(file_path) {
        Ok(file) => file,
        Err(e) => {
            println!("cannot read file {file_path}: {e}");
            std::process::exit(1)
        }
    };

    let mut contents = String::new();
    match file_data.read_to_string(&mut contents) {
        Ok(_) => contents,
        Err(e) => {
            println!("cannot read file {file_path}: {e}");
            std::process::exit(1);
        }
    }
}

pub fn handle_include_error(
    fname: &str,
    loc: &std::ops::Range<usize>,
    error_count: &mut i32,
    message: &str,
    help: Option<String>,
) {
    let problem = ParserError {
        file: fname.to_string(),
        help,
        input: read_file(fname),
        message: message.to_string(),
        start_pos: loc.start,
        last_pos: loc.end,
    };
    *error_count += 1;
    println!("{problem}\n");
}
