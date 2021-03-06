use std::rc::Rc;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct SyntaxToken {
    raw: Rc<RawSyntaxTokenData>,
    start: usize,
    len: usize,
}

impl SyntaxToken {
    pub fn new(raw: Rc<RawSyntaxTokenData>, start: usize, len: usize) -> Self {
        Self { raw, start, len }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct RawSyntaxTokenData {
    kind: SyntaxTokenKind,
    text: String,
}

impl RawSyntaxTokenData {
    pub fn new<S>(kind: SyntaxTokenKind, text: S) -> Self
    where
        S: Into<String>,
    {
        Self { kind, text: text.into() }
    }
}

#[allow(dead_code)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum SyntaxTokenKind {
    Identifier,
    Keyword(Keyword),
    Literal(Literal),
    Symbol(Symbol),
}

#[allow(dead_code)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Keyword {
    Def,
    Let,
}

#[allow(dead_code)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Literal {
    Char,
    Float,
    Integer,
    String,
}

#[allow(dead_code)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Symbol {
    Asterisk,
    Eq,
    Minus,
    Plus,
    ForwardSlash,
    LParen,
    RParen,
}

#[test]
fn test_syntax_token() {
    println!("{}", std::mem::size_of::<SyntaxToken>());
    println!("{}", std::mem::size_of::<SyntaxTokenKind>());
}
