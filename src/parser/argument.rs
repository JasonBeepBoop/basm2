use crate::*;
impl Parser<'_> {
    pub fn parse_argument(&self, token: TokenKind) -> InstructionArgument {
        match token {
            TokenKind::Mem(inner) => InstructionArgument::Mem(inner),
            TokenKind::Imm(num)
            | TokenKind::HexLit(num)
            | TokenKind::BinLit(num)
            | TokenKind::IntLit(num)
            | TokenKind::OctLit(num) => InstructionArgument::Imm(num),
            TokenKind::IReg(reg) => InstructionArgument::IReg(reg),
            TokenKind::Register(reg) => InstructionArgument::Reg(reg),
            TokenKind::IMem(inner) => InstructionArgument::IMem(inner),
            TokenKind::Ident(ident) => InstructionArgument::Ident(ident),
            TokenKind::MacroIdent(ident) => InstructionArgument::MacroIdent(ident),
            TokenKind::Expr(num) => InstructionArgument::Imm(num),
            _ => InstructionArgument::Imm(0),
        }
    }
}
