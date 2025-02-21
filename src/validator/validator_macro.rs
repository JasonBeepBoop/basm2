use crate::*;
impl MacroContent {
    pub fn is_valid(
        &self,
        orig_data: String,
        toks: Vec<(TokenKind, std::ops::Range<usize>)>,
    ) -> Result<(), MacroValidatorError> {
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
                token if token.is_imm() => Some(ArgumentType::Imm),
                TokenKind::Comma => None,
                _ => {
                    return Err(MacroValidatorError {
                        err_input: self.full_data.to_string(),
                        err_message: format!("a {token} is not a valid macro argument"),
                        help: None,
                        orig_input: orig_data.to_string(),
                        orig_pos: span,
                        mac: self,
                    })
                }
            };
            if let Some(v) = data {
                parsed_toks.push((v, span));
            }
        }
        let mut current_args = Vec::new();
        for (arg, e) in &self.args {
            current_args.push((arg.arg_type.clone(), e));
        }
        for (index, (arg, span)) in self.args.iter().enumerate() {
            if let Some((d, span)) = parsed_toks.get(index) {
                if *d == arg.arg_type {
                    continue;
                } else {
                    return Err(MacroValidatorError {
                        err_input: self.full_data.to_string(),
                        err_message: format!("expected a {}, found a {d}", arg.arg_type),
                        help: None,
                        orig_input: orig_data.to_string(),
                        orig_pos: span.clone(),
                        mac: self,
                    });
                }
            } else {
                return Err(MacroValidatorError {
                    err_input: self.full_data.to_string(),
                    err_message: String::from("an incorrect number of arguments were supplied"),
                    help: None,
                    orig_input: orig_data.to_string(),
                    orig_pos: span.clone(),
                    mac: self,
                });
            }
        }
        Ok(())
    }
}
