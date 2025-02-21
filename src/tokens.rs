use crate::*;
use logos::Logos;
use serde::Serialize;

#[derive(Logos, Debug, Clone, PartialEq, Serialize)]
pub enum TokenKind {
    #[token("\n")]
    Newline,

    #[token(" ", logos::skip)]
    Whitespace,

    #[token("\t", logos::skip)]
    Tab,

    #[token("(")]
    LeftParen,

    #[token(")")]
    RightParen,

    #[token("{")]
    LeftBrace,

    #[token("}")]
    RightBrace,

    #[token(",")]
    Comma,

    #[token("~")]
    Tilde,

    #[token("`")]
    Grave,

    #[token("#")]
    Pound,

    #[token("+")]
    Plus,

    #[token("++")]
    PlusPlus,

    #[token("-")]
    Minus,

    #[token("--")]
    MinusMinus,

    #[token("*")]
    Star,

    #[token("/")]
    Slash,

    #[token("%")]
    Mod,

    #[token("!")]
    Bang,

    #[token("=")]
    Equal,

    #[token(">")]
    Greater,

    #[token(">>")]
    GreaterGreater,

    #[token("<")]
    Less,

    #[token("<<")]
    LessLess,

    #[token("&")]
    Amp,

    #[token("&&")]
    AmpAmp,

    #[token("|")]
    Pipe,

    #[token("[")]
    LeftBracket,

    #[token("]")]
    RightBracket,

    #[token("||")]
    PipePipe,

    #[token("^")]
    Xor,

    #[token(":")]
    Colon,

    #[regex("[rR][0-9]", |lex| lex.slice()[1..].parse::<u8>().unwrap())]
    Register(u8),

    #[regex(r"'([^\\']|\\.)'", |lex| parse_char(lex.slice()))]
    CharLit(char),

    #[regex(r#""([^\\"]|\\.)*""#, |lex| parse_string(lex.slice()))]
    StringLit(String),

    #[regex(r"(?:0[bB][01]+|0[oO][0-7]+|0[xX][0-9a-fA-F]+|\d+)", |lex| parse_content(lex.slice()))]
    IntLit(i64),

    #[regex(r"macro_rules!", |lex| lex.slice().to_string())]
    MacroDef(String),

    #[regex("const[ ][a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice()[6..].trim().to_string())]
    Constant(String),

    #[regex("[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Ident(String),

    #[regex(r"\.[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Directive(String),

    #[regex("%[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice()[1..].to_string())]
    MacroIdent(String),

    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*!\(", |lex| lex.slice()[0..lex.slice().len() - 2].to_string())]
    MacroCall(String),

    #[regex("%[a-zA-Z_][a-zA-Z0-9_]*:", |lex| lex.slice()[1..lex.slice().len() - 1].to_string())]
    MacroLabel(String),

    #[regex(";.*", logos::skip)]
    Comment,

    Macro(MacroContent),

    Instruction(InstructionData),

    Label(String),

    Mem(MemAddr),

    IIdent(String),

    #[regex("&[rR][0-9]", |lex| lex.slice()[2..].parse::<u8>().unwrap())]
    IReg(u8),
    Imm(i64),
    Expr(i64),
}
fn parse_content(content: &str) -> i64 {
    if content.starts_with("0x") || content.starts_with("0X") {
        i64::from_str_radix(&content[2..], 16).unwrap()
    } else if content.starts_with("0b") || content.starts_with("0B") {
        i64::from_str_radix(&content[2..], 2).unwrap()
    } else if content.starts_with("0o") || content.starts_with("0O") {
        i64::from_str_radix(&content[2..], 8).unwrap()
    } else if content.chars().all(|c| c.is_ascii_digit()) {
        content.parse::<i64>().unwrap()
    } else {
        panic!("lexer failed to parse Integer Literal");
    }
}

impl TokenKind {
    pub fn is_empty(&self) -> bool {
        matches!(self, TokenKind::Tab | TokenKind::Whitespace)
    }
}
fn parse_char(s: &str) -> char {
    let inner = &s[1..s.len() - 1];
    match inner {
        "\\n" => '\n',
        "\\r" => '\r',
        "\\t" => '\t',
        "\\0" => '\0',
        "\\'" => '\'',
        "\\\"" => '\"',
        "\\\\" => '\\',
        _ if inner.len() == 1 => inner.chars().next().unwrap(),
        _ => panic!("Invalid character escape sequence: {}", s),
    }
}
fn parse_string(s: &str) -> String {
    let inner = &s[1..s.len() - 1];
    let mut result = String::new();
    let mut chars = inner.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next() {
                Some('n') => result.push('\n'),
                Some('r') => result.push('\r'),
                Some('t') => result.push('\t'),
                Some('0') => result.push('\0'),
                Some('\'') => result.push('\''),
                Some('"') => result.push('\"'),
                Some('\\') => result.push('\\'),
                _ => panic!("Invalid string escape sequence"),
            }
        } else {
            result.push(c);
        }
    }

    result
}
