use core::fmt;

use chumsky::{prelude::*, text};

pub fn tokenize() -> impl Parser<char, Vec<Token>, Error = Simple<char>> {
    let token = choice((
        choice([
            text::keyword("break").to(Token::Break),
            text::keyword("else").to(Token::Else),
            text::keyword("enum").to(Token::Enum),
            text::keyword("fn").to(Token::Fn),
            text::keyword("for").to(Token::For),
            text::keyword("if").to(Token::If),
            text::keyword("in").to(Token::In),
            text::keyword("let").to(Token::Let),
            text::keyword("mut").to(Token::Mut),
            text::keyword("return").to(Token::Return),
            text::keyword("struct").to(Token::Struct),
        ]),
        choice([
            just("->").to(Token::RightArrow),
            just("..").to(Token::Dot2),
            just("::").to(Token::Colon2),
            just("==").to(Token::Eq2),
            just("&&").to(Token::And2),
            just("+").to(Token::Plus),
            just("-").to(Token::Minus),
            just(".").to(Token::Dot),
            just(",").to(Token::Comma),
            just(":").to(Token::Colon),
            just(";").to(Token::Semicolon),
            just("=").to(Token::Eq),
            just("!").to(Token::Exclam),
            just("(").to(Token::OpenParen),
            just(")").to(Token::CloseParen),
            just("[").to(Token::OpenBracket),
            just("]").to(Token::CloseBracket),
            just("{").to(Token::OpenBrace),
            just("}").to(Token::CloseBrace),
        ]),
        text::ident().map(|s| Token::Ident(Ident(s))),
        text::int(10).map(|s| Token::IntLiteral(IntLiteral(s))),
    ));

    let line_comment = just("//").then(take_until(text::newline())).ignored();
    let block_comment = just("/*").then(take_until(just("*/"))).ignored();

    let whitespace = choice((
        text::whitespace().at_least(1).ignored(),
        line_comment,
        block_comment,
    ))
    .repeated();

    (whitespace.ignore_then(token))
        .repeated()
        .then_ignore(whitespace)
        .then_ignore(end())
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Token {
    Ident(Ident),
    IntLiteral(IntLiteral),
    Break,
    Else,
    Enum,
    Fn,
    For,
    If,
    In,
    Let,
    Mut,
    Return,
    Struct,
    RightArrow,
    Plus,
    Minus,
    Colon,
    Colon2,
    Semicolon,
    Dot,
    Dot2,
    Comma,
    Eq,
    Eq2,
    And2,
    Exclam,
    OpenParen,
    CloseParen,
    OpenBracket,
    CloseBracket,
    OpenBrace,
    CloseBrace,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let val: &dyn fmt::Display = match self {
            Self::Ident(x) => x,
            Self::IntLiteral(x) => x,
            Self::Break => &"break",
            Self::Else => &"else",
            Self::Enum => &"enum",
            Self::Fn => &"fn",
            Self::For => &"for",
            Self::If => &"if",
            Self::In => &"in",
            Self::Let => &"let",
            Self::Mut => &"mut",
            Self::Return => &"return",
            Self::Struct => &"struct",
            Self::RightArrow => &"->",
            Self::Plus => &"+",
            Self::Minus => &"-",
            Self::Colon => &":",
            Self::Colon2 => &"::",
            Self::Semicolon => &";",
            Self::Dot => &".",
            Self::Dot2 => &"..",
            Self::Comma => &",",
            Self::Eq => &"=",
            Self::Eq2 => &"==",
            Self::And2 => &"&&",
            Self::Exclam => &"!",
            Self::OpenParen => &"(",
            Self::CloseParen => &")",
            Self::OpenBracket => &"[",
            Self::CloseBracket => &"]",
            Self::OpenBrace => &"{",
            Self::CloseBrace => &"}",
        };
        write!(f, "{}", val)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Ident(String);

impl fmt::Display for Ident {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IntLiteral(String);

impl fmt::Display for IntLiteral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
