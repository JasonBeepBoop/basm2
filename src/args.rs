use crate::*;
use serde::Serialize;
use std::fmt;

#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct MacroContent {
    pub name: String,
    pub args: Vec<(FullArgument, std::ops::Range<usize>)>,
    pub tokens: Vec<(TokenKind, std::ops::Range<usize>)>,
}

#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct FullArgument {
    pub name: String,
    pub arg_type: ArgumentType,
}

#[derive(Debug, PartialEq, Serialize, Clone)]
pub enum ArgumentType {
    Mem,
    Imem,
    Ireg,
    Imm,
    Reg,
    Label,
}

impl ArgumentType {
    pub fn from_string(s: &str) -> Option<Self> {
        match s {
            "mem" => Some(ArgumentType::Mem),
            "imem" => Some(ArgumentType::Imem),
            "ireg" => Some(ArgumentType::Ireg),
            "imm" => Some(ArgumentType::Imm),
            "reg" => Some(ArgumentType::Reg),
            "label" => Some(ArgumentType::Label),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Clone)]
pub enum InstructionArgument {
    Mem(Box<TokenKind>),
    IMem(Box<TokenKind>),
    Reg(u8),
    IReg(u8),
    Imm(i64),
    Ident(String),
    MacroIdent(String),
}

#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct InstructionData {
    pub name: String,
    pub args: Vec<(InstructionArgument, std::ops::Range<usize>)>,
}

impl fmt::Display for MacroContent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Macro: {}", self.name)?;
        writeln!(f, "├── Args:")?;
        for (i, (arg, _)) in self.args.iter().enumerate() {
            if i != self.args.len() - 1 {
                writeln!(f, "│   ├── {}", arg)?;
            } else {
                writeln!(f, "│   └── {}", arg)?;
            }
        }
        writeln!(f, "└── Tokens:")?;
        for (i, (token, _)) in self.tokens.iter().enumerate() {
            if i != self.tokens.len() - 1 {
                writeln!(f, "    ├── {}", token)?;
            } else {
                write!(f, "    └── {}", token)?;
            }
        }
        Ok(())
    }
}

impl fmt::Display for FullArgument {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.arg_type)
    }
}

impl fmt::Display for ArgumentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ArgumentType::Mem => write!(f, "Mem"),
            ArgumentType::Imem => write!(f, "Imem"),
            ArgumentType::Ireg => write!(f, "Ireg"),
            ArgumentType::Imm => write!(f, "Imm"),
            ArgumentType::Reg => write!(f, "Reg"),
            ArgumentType::Label => write!(f, "Label"),
        }
    }
}

impl fmt::Display for InstructionArgument {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InstructionArgument::Mem(token) => write!(f, "Mem({})", token),
            InstructionArgument::IMem(token) => write!(f, "IMem({})", token),
            InstructionArgument::Reg(reg) => write!(f, "Reg({})", reg),
            InstructionArgument::IReg(reg) => write!(f, "IReg({})", reg),
            InstructionArgument::Imm(imm) => write!(f, "Imm({})", imm),
            InstructionArgument::Ident(ident) => write!(f, "Ident({})", ident),
            InstructionArgument::MacroIdent(ident) => write!(f, "MacroIdent({})", ident),
        }
    }
}

impl fmt::Display for InstructionData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Instruction: {}", self.name)?;
        writeln!(f, "    └─┐ Args:")?;
        for (i, (arg, _)) in self.args.iter().enumerate() {
            if i != self.args.len() - 1 {
                writeln!(f, "      ├── {}", arg)?;
            } else {
                write!(f, "    ┌─┴── {}", arg)?;
            }
        }
        Ok(())
    }
}
