use crate::*;
impl MacroContent {
    pub fn is_valid(&self, toks: Vec<(TokenKind, std::ops::Range<usize>)>) -> bool {
        // okay... here, I need to check first if the token types of the input
        // match the tokens inside of the macro.
        // what I can do, is I can iterate through the input tokens, and iterate through the arguments
        let mut parsed_toks = Vec::new();
        for (token, span) in toks {
            // this loop will clean up the toks and parse it into types
            let data = match token {
                token if token.is_reg() => Some(ArgumentType::Reg),
                token if token.is_ireg() => Some(ArgumentType::Ireg),
                token if token.is_mem() => Some(ArgumentType::Mem),
                token if token.is_imem() => Some(ArgumentType::Imem),
                token if token.is_imm() => Some(ArgumentType::Imm),
                TokenKind::Comma => None,
                _ => panic!(":3"),
            };
            if let Some(v) = data {
                parsed_toks.push((v, span));
            }
        }
        let mut current_args = Vec::new();
        for (arg, e) in &self.args {
            current_args.push((arg.arg_type.clone(), e));
        }
        for (index, (arg, _)) in self.args.iter().enumerate() {
            if let Some((d, _)) = parsed_toks.get(index) {
                if *d == arg.arg_type {
                    continue;
                } else {
                    panic!();
                }
            } else {
                panic!();
            }
        }
        true
    }
}
