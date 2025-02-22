use crate::*;
use colored::*;
impl InstructionData {
    pub fn is_valid(&self) -> Result<(), String> {
        // Ident is for matching labels - they will be memory addresses
        let (ins_class, valid_args) = match self.name.to_lowercase().as_str() {
            "add" | "mov" | "nand" | "div" | "cmp" => (
                0,
                self.args.len() == 2
                    && self.args.first().is_some_and(|x| x.0.is_reg())
                    && self.args.get(1).is_some_and(|expr| {
                        expr.0.is_reg() || expr.0.is_ireg() || expr.0.is_imm() || expr.0.is_imem()
                    }),
            ),
            "jmp" | "bo" | "bno" | "bg" | "bl" | "bz" | "bnz" => (
                1,
                self.args.len() == 1
                    && self
                        .args
                        .first()
                        .is_some_and(|arg| arg.0.is_ident() || arg.0.is_mem() || arg.0.is_ireg()),
            ),
            "ret" | "hlt" => (2, self.args.is_empty()),
            "ld" | "lea" => (
                3,
                self.args.len() == 2
                    && self.args.first().is_some_and(|x| x.0.is_reg())
                    && self
                        .args
                        .get(1)
                        .is_some_and(|expr| expr.0.is_mem() || expr.0.is_ident()),
            ),
            "st" => (
                4,
                self.args.len() == 2
                    && self
                        .args
                        .first()
                        .is_some_and(|x| x.0.is_mem() || x.0.is_ireg() || x.0.is_ident())
                    && self.args.get(1).is_some_and(|expr| expr.0.is_reg()),
            ),
            "int" => (
                5,
                self.args.len() == 1 && self.args.first().is_some_and(|x| x.0.is_imm()),
            ),
            _ => {
                return Err(format!(
                    "instruction {} does not exist",
                    self.name.to_uppercase()
                ))
            }
        };
        let lhs = if let Some((v, _)) = self.args.first() {
            v.get_raw()
        } else {
            String::from("no")
        };
        let rhs = if let Some((v, _)) = self.args.get(1) {
            v.get_raw()
        } else {
            String::from("no")
        };

        if valid_args {
            return Ok(());
        }
        Err(format!(
            "{}: {} requires {} LHS and {} RHS\n{}: found {lhs} LHS and {rhs} RHS\n ",
            "invalid instruction".bold(),
            self.name.to_uppercase().magenta(),
            LHS_DETAIL[ins_class].cyan(),
            RHS_DETAIL[ins_class].cyan(),
            "note".yellow(),
        ))
    }
}

const LHS_DETAIL: [&str; 6] = [
    "register",
    "memory address or register indirect",
    "no",
    "register",
    "memory address or register indirect",
    "immediate",
];
const RHS_DETAIL: [&str; 6] = [
    "register, register indirect, memory indirect, or immediate",
    "no",
    "no",
    "memory address",
    "register",
    "no",
];
