use crate::*;
use serde::Serialize;
use std::fmt;

#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct MacroContent {
    pub full_data: String,
    pub file: String,
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
    pub fn equals(&self, t: TokenKind) -> bool {
        use crate::ArgumentType::*;
        (*self == Mem && t.is_mem())
            || (*self == Reg && t.is_reg())
            || (*self == Ireg && t.is_ireg())
            || (*self == Imem && t.is_imem())
            || (*self == Label && t.is_ident())
    }
}

impl TokenKind {
    pub fn is_imm(&self) -> bool {
        matches!(self, TokenKind::IntLit(_))
    }
    pub fn is_mem(&self) -> bool {
        matches!(self, TokenKind::Mem(_))
    }
    pub fn is_imem(&self) -> bool {
        matches!(self, TokenKind::IMem(_))
    }
    pub fn is_reg(&self) -> bool {
        matches!(self, TokenKind::Register(_))
    }
    pub fn is_ireg(&self) -> bool {
        matches!(self, TokenKind::IReg(_))
    }
    pub fn is_ident(&self) -> bool {
        matches!(self, TokenKind::Ident(_))
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
            ArgumentType::Mem => write!(f, "memory direct"),
            ArgumentType::Imem => write!(f, "memory indirect"),
            ArgumentType::Ireg => write!(f, "register indirect"),
            ArgumentType::Imm => write!(f, "immediate"),
            ArgumentType::Reg => write!(f, "register"),
            ArgumentType::Label => write!(f, "label"),
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
