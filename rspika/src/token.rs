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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Ident(String);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IntLiteral(String);
