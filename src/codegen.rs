use crate::*;

pub const HLT_OP: i16 = 0b0000;
pub const ADD_OP: i16 = 0b0001;
pub const BO_OP: i16 = 0b00100;
pub const BNO_OP: i16 = 0b00101;
pub const POP_OP: i16 = 0b0011;
pub const DIV_OP: i16 = 0b0100;
pub const RET_OP: i16 = 0b0101;
pub const BL_OP: i16 = 0b01010;
pub const BG_OP: i16 = 0b01011;
pub const LD_OP: i16 = 0b0110;
pub const ST_OP: i16 = 0b0111;
pub const JMP_OP: i16 = 0b10000;
pub const BZ_OP: i16 = 0b10010;
pub const BNZ_OP: i16 = 0b10011;
pub const CMP_OP: i16 = 0b1010;
pub const NAND_OP: i16 = 0b1011;
pub const PUSH_OP: i16 = 0b1100;
pub const INT_OP: i16 = 0b1101;
pub const MOV_OP: i16 = 0b1110;
pub const LEA_OP: i16 = 0b1111;

const HLT_TYPE: u8 = 0;
const MOV_TYPE: u8 = 1;
const B_TYPE: u8 = 2;
const POP_TYPE: u8 = 3;
const LD_TYPE: u8 = 4;
const ST_TYPE: u8 = 5;
type CodeGenError = ParserError;
use std::ops::Range;
pub fn encode(
    ins: &TokenKind,
    next_ins: Option<&(String, TokenKind, Range<usize>)>,
) -> Result<Vec<i16>, CodeGenError> {
    let mut encoded_tokens = Vec::new();
    match ins {
        TokenKind::Instruction(ins) => {
            let (opcode, ins_class) = match ins.name.to_lowercase().as_str() {
                // all instructions should be valid when this is reached, as it is validated in
                // validator/validator_ins.rs. therefore, it is fine to use MOV_TYPE for PUSH and INT
                // because the argument types and counts are validated.
                //
                //
                // But, I do need to check labels as they are not detected in previous instances
                //
                // I should also allow it to reference CONSTs too, and see if it is talking about a label
                // or a CONST (check labels first)
                //
                // ^^^^ I need to validate that there are no duplicate names across symbol tabels for
                // labels and constants
                "hlt" => (HLT_OP, HLT_TYPE),
                "add" => (ADD_OP, MOV_TYPE),
                "bo" => (BO_OP, B_TYPE),
                "bno" => (BNO_OP, B_TYPE),
                "pop" => (POP_OP, POP_TYPE),
                "div" => (DIV_OP, MOV_TYPE),
                "ret" => (RET_OP, HLT_TYPE),
                "bl" => (BL_OP, B_TYPE),
                "bg" => (BG_OP, B_TYPE),
                "ld" => (LD_OP, LD_TYPE),
                "st" => (ST_OP, ST_TYPE),
                "jmp" => (JMP_OP, B_TYPE),
                "bz" => (BZ_OP, B_TYPE),
                "bnz" => (BNZ_OP, B_TYPE),
                "cmp" => (CMP_OP, MOV_TYPE),
                "nand" => (NAND_OP, MOV_TYPE),
                "push" => (PUSH_OP, MOV_TYPE),
                "int" => (INT_OP, MOV_TYPE),
                "mov" => (MOV_OP, MOV_TYPE),
                "lea" => (LEA_OP, LD_TYPE),
                _ => panic!(":3"),
            };
            match encode_instruction(&opcode, &ins_class, &ins.args) {
                Ok(v) => encoded_tokens.push(v),
                Err(e) => return Err(e),
            }
        }
        _ => todo!(),
    }

    Ok(vec![])
}

fn encode_instruction(
    opcode: &i16,
    class: &u8,
    args: &[(InstructionArgument, std::ops::Range<usize>)],
) -> Result<i16, CodeGenError> {
    let lhs = args.first();
    let rhs = args.get(1);
    let mut encoded = 0;
    let l_map = LABEL_MAP.lock().unwrap();
    match *class {
        MOV_TYPE => {
            encoded = opcode << 12;
            if let Some((InstructionArgument::Reg(r), _)) = lhs {
                encoded |= (*r as i16) << 9;
            } else {
                panic!("oops");
            }
            if let Some((arg, _)) = rhs {
                match arg {
                    InstructionArgument::Reg(r) => encoded |= *r as i16,
                    InstructionArgument::IReg(r) => encoded = encoded | (1 << 6) | (*r as i16),
                    InstructionArgument::Mem(m) => {
                        if m.is_indirect() {
                            if let Some(v) = m.content.first() {
                                encoded = encoded | (1 << 7) | (v.0.get_value() as i16);
                            } else {
                                panic!();
                            }
                        } else {
                            panic!("when the:");
                        }
                    }
                    InstructionArgument::Imm(l) => encoded = encoded | (1 << 8) | arg.get_imm(),
                    _ => panic!("yah"),
                }
            } else {
                panic!("silly me");
            }
        }
        B_TYPE => {
            encoded = opcode << 11;
            use crate::InstructionArgument::*;
            match &lhs.unwrap().0 {
                // safe to unwrap - checked earlier
                Ident(i) => {
                    if let Some((name, span, value)) = l_map.get(i) {
                        if *value >= 1024 {
                            panic!("too big");
                        } else {
                            encoded |= *value as i16;
                        }
                    } else {
                        panic!("nope can't find ident");
                    }
                }
                Mem(m) => {
                    if !m.is_indirect() {
                        if let Some(v) = m.content.first() {
                            encoded |= v.0.get_value() as i16;
                        } else {
                            panic!("memory should be direct");
                        }
                    } else {
                        panic!("when the:");
                    }
                }
                IReg(r) => {
                    encoded = encoded | (1 << 10) | (*r as i16);
                }
                _ => panic!("well well"),
            }
        }
        POP_TYPE => {}
        LD_TYPE => {}
        ST_TYPE => {}
        _ => panic!(),
    }
    Ok(encoded)
}
