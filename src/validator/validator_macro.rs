use crate::*;
use std::collections::HashMap;

#[allow(suspicious_double_ref_op)]
impl MacroContent {
    pub fn is_valid(
        &self,
        orig_data: String,
        toks: Vec<(TokenKind, std::ops::Range<usize>)>, // incoming macro args
    ) -> Result<Vec<(TokenKind, std::ops::Range<usize>)>, Vec<MacroValidatorError>> {
        // okay... here, I need to check first if the token types of the input
        // match the tokens inside of the macro.
        // what I can do, is I can iterate through the input tokens, and iterate through the arguments
        let mut parsed_toks = Vec::new();
        let mut argument_indices = Vec::new();
        let mut errs = Vec::new();
        for (index, (token, span)) in toks.iter().enumerate() {
            // this loop will clean up the toks and parse it into types
            let data = match token {
                token if token.is_reg() => Some(ArgumentType::Reg),
                token if token.is_ireg() => Some(ArgumentType::Ireg),
                token if token.is_mem() => Some(ArgumentType::Mem),
                token if token.is_imem() => Some(ArgumentType::Imem),
                token if token.is_imm() => Some(ArgumentType::Imm),
                token if token.is_ident() => Some(ArgumentType::Label),
                TokenKind::Comma => None,
                _ => {
                    errs.push(MacroValidatorError {
                        err_input: self.full_data.to_string(),
                        err_message: format!("a {token} is not a valid macro argument"),
                        help: None,
                        orig_input: orig_data.to_string(),
                        orig_pos: span.clone(),
                        mac: self.clone(),
                    });
                    break;
                }
            };
            if let Some(v) = data {
                parsed_toks.push((v, span));
                argument_indices.push(index);
            }
        }
        let mut current_args = Vec::new();
        for (arg, e) in &self.args {
            current_args.push((arg.arg_type.clone(), e));
        }
        let f = if let Some((_, s)) = parsed_toks.first() {
            s
        } else {
            &&(0..0)
        };
        if parsed_toks.len() != self.args.len() {
            errs.push(MacroValidatorError {
                err_input: self.full_data.to_string(),
                err_message: format!(
                    "expected {} arguments, found {}",
                    self.args.len(),
                    parsed_toks.len()
                ),
                help: None,
                orig_input: orig_data.to_string(),
                orig_pos: f.clone().clone(),
                mac: self.clone(),
            });
        }
        for (index, (arg, span)) in self.args.iter().enumerate() {
            if let Some((d, span)) = parsed_toks.get(index) {
                if *d == arg.arg_type {
                    continue;
                } else {
                    errs.push(MacroValidatorError {
                        err_input: self.full_data.to_string(),
                        err_message: format!("expected {}, found {d}", arg.arg_type),
                        help: None,
                        orig_input: orig_data.to_string(),
                        orig_pos: span.clone().clone(),
                        mac: self.clone(),
                    });
                    break;
                }
            } else {
                errs.push(MacroValidatorError {
                    err_input: self.full_data.to_string(),
                    err_message: String::from("an incorrect number of arguments were supplied"),
                    help: None, // borrow checker is yappin
                    orig_input: orig_data.to_string(),
                    orig_pos: span.clone(),
                    mac: self.clone(),
                });
                break;
            }
        } // we need a hashmap of type ident names, TokenKind to record arguments
        let mut arg_map: HashMap<&String, &crate::TokenKind> = HashMap::new();
        let mut count = 0;
        for element in argument_indices {
            // we no longer need to keep track of argument locations
            if let Some((v, _)) = toks.get(element) {
                if let Some((l, _)) = self.args.get(count) {
                    arg_map.insert(&l.name, v);
                    count += 1;
                }
            }
        }
        let mut new_elems = Vec::new();
        for (element, span) in &self.tokens {
            if let TokenKind::MacroIdent(name) = element {
                if let Some(v) = arg_map.get(&name) {
                    new_elems.push((v.clone().clone(), span.clone()));
                    continue;
                } else {
                    panic!("this is not a macro arg: {name}");
                }
            } else if let TokenKind::Instruction(contents) = element {
                let mut ins_args = Vec::new();
                for (thing, place) in &contents.args {
                    if let InstructionArgument::MacroIdent(name) = thing {
                        if let Some(v) = arg_map.get(&name) {
                            ins_args.push((v.to_tok_kind(), span.clone()));
                            continue;
                        } else {
                            panic!("this is not a macro arg: {name}");
                        }
                    }
                    ins_args.push((thing.clone(), place.clone()));
                }
                let reconstruct = InstructionData {
                    expanded: true,
                    name: contents.name.to_string(),
                    args: ins_args,
                };
                if let Err(e) = reconstruct.is_valid() {
                    errs.push(MacroValidatorError {
                        err_input: self.full_data.to_string(),
                        err_message: e,
                        help: None,
                        orig_input: orig_data.to_string(),
                        orig_pos: span.clone(),
                        mac: self.clone(),
                    });
                }
                new_elems.push((TokenKind::Instruction(reconstruct), span.clone()));
                continue;
            }
            new_elems.push((element.clone(), span.clone()));
        }
        if !errs.is_empty() {
            return Err(errs);
        }
        Ok(new_elems)
    }
}
