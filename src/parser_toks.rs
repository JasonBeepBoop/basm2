use crate::*;
use serde::Serialize;
use std::fmt;

#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct MemAddr {
    pub indirect: bool,
    pub content: Vec<(TokenKind, std::ops::Range<usize>)>,
}

#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct MacroContent {
    pub full_data: String,
    pub file: String,
    pub name: (String, std::ops::Range<usize>),
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
    // for macros
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
            || (*self == Label && t.is_ident())
    }
}

impl TokenKind {
    pub fn is_imm(&self) -> bool {
        matches!(self, TokenKind::IntLit(_))
    }
    pub fn is_mem(&self) -> bool {
        matches!(self, TokenKind::Mem(mem_addr) if !mem_addr.indirect)
    }
    pub fn is_imem(&self) -> bool {
        matches!(self, TokenKind::Mem(mem_addr) if mem_addr.indirect)
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
    Mem(MemAddr),
    Reg(u8),
    IReg(u8),
    Imm(i64),
    Ident(String),
    MacroIdent(String),
}
impl InstructionArgument {
    pub fn get_value(&self) -> i64 {
        use crate::InstructionArgument::*;
        match self {
            Reg(v) => *v as i64,
            IReg(v) => *v as i64,
            Imm(v) => *v,
            Ident(s) => {
                let vmap = V_MAP.lock().unwrap();
                if let Some((_, _, v)) = vmap.get(s) {
                    *v
                } else {
                    0
                }
            }
            Mem(m) => {
                if let Some((v, _)) = m.content.first() {
                    v.get_value()
                } else {
                    0
                }
            }
            _ => 0,
        }
    }
    pub fn is_imm(&self) -> bool {
        matches!(self, InstructionArgument::Imm(_))
    }
    pub fn is_mem(&self) -> bool {
        matches!(self, InstructionArgument::Mem(mem_addr) if !mem_addr.indirect)
    }
    pub fn is_imem(&self) -> bool {
        matches!(self, InstructionArgument::Mem(mem_addr) if mem_addr.indirect)
    }

    pub fn is_reg(&self) -> bool {
        matches!(self, InstructionArgument::Reg(_))
    }
    pub fn is_ireg(&self) -> bool {
        matches!(self, InstructionArgument::IReg(_))
    }
    pub fn is_ident(&self) -> bool {
        matches!(
            self,
            InstructionArgument::Ident(_) | InstructionArgument::MacroIdent(_)
        )
    }
}
impl TokenKind {
    pub fn to_tok_kind(&self) -> InstructionArgument {
        use crate::TokenKind::*;
        match self {
            Mem(v) => InstructionArgument::Mem(v.clone()),
            Register(v) => InstructionArgument::Reg(*v),
            IReg(v) => InstructionArgument::IReg(*v),
            IntLit(v) => InstructionArgument::Imm(*v),
            Ident(v) => InstructionArgument::Ident(v.clone()),
            MacroIdent(v) => InstructionArgument::MacroIdent(v.clone()),
            _ => panic!(":3"),
        }
    }
    pub fn get_value(&self) -> i64 {
        match self {
            TokenKind::IntLit(v) => *v,
            _ => 0,
        }
    }
}

impl InstructionArgument {
    pub fn to_tok_kind(&self) -> TokenKind {
        use crate::InstructionArgument::*;
        match self {
            Mem(v) => TokenKind::Mem(v.clone()),
            Reg(v) => TokenKind::Register(*v),
            IReg(v) => TokenKind::IReg(*v),
            Imm(v) => TokenKind::IntLit(*v),
            Ident(v) => TokenKind::Ident(v.clone()),
            MacroIdent(v) => TokenKind::MacroIdent(v.clone()),
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct InstructionData {
    pub expanded: bool,
    pub name: String,
    pub args: Vec<(InstructionArgument, std::ops::Range<usize>)>,
}

impl fmt::Display for MemAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "├── Indirect: {}", self.indirect)?;
        for (i, (arg, _)) in self.content.iter().enumerate() {
            if i != self.content.len() - 1 {
                writeln!(f, "    │   ├── {}", arg)?;
            } else {
                write!(f, "    │   └── {}", arg)?;
            }
        }
        Ok(())
    }
}

impl fmt::Display for MacroContent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Macro: {}", self.name.0)?;
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
impl InstructionArgument {
    pub fn get_raw(&self) -> String {
        match self {
            InstructionArgument::Mem(_) => String::from("memory direct"),
            InstructionArgument::Reg(_) => String::from("register"),
            InstructionArgument::IReg(_) => String::from("register indirect"),
            InstructionArgument::Imm(_) => String::from("immediate"),
            InstructionArgument::Ident(_) => String::from("identifier"),
            InstructionArgument::MacroIdent(_) => String::from("macro identifier"),
        }
    }
}
impl fmt::Display for InstructionArgument {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InstructionArgument::Mem(token) => write!(f, "Mem\n    {}", token),
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
        writeln!(
            f,
            "Instruction: {}, Expanded from macro {}",
            self.name, self.expanded
        )?;
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
