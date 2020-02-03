use super::ast::*;

pub fn run(i: &str) -> nom::IResult<&str, Vec<AstNode>> {
    nom::multi::many0(lex_statement)(i)
}

pub fn lex_statement(i: &str) -> nom::IResult<&str, AstNode> {
    lex_function(i)
}

fn lex_function_typedef(i: &str) -> nom::IResult<&str, FunctionTypedef> {
    Ok((i, FunctionTypedef {
        parameter_type: Identifier { identifier: "".to_string() },
        return_type: Identifier { identifier: "".to_string() }
    }))
}

fn lex_function_body(i: &str) -> nom::IResult<&str, FunctionBody> {
    Ok((i, FunctionBody { }))
}

fn lex_function(i: &str) -> nom::IResult<&str, AstNode> {
    match lex_function_typedef(i) {
        Ok((i, typedef)) => {
            match lex_function_body(i) {
                Ok((i, body)) => {
                    Ok((i, AstNode { kind: AstNodeKind::Function(Some(typedef), body) }))
                },
                Err(e) => Err(e)
            }
        },
        Err(_) => {
            match lex_function_body(i) {
                Ok((i, body)) => {
                    Ok((i, AstNode { kind: AstNodeKind::Function(None, body) }))
                },
                Err(e) => Err(e)
            }
        }
    }
}

/*
pub fn lex_token(i: &str) -> nom::IResult<&str, Token> {
    match nom::sequence::tuple((
        skip_ws,
        nom::branch::alt((
                lex_assign,
                lex_identifier,
                lex_typedef,
                lex_arrow,
                lex_unbound,
                lex_period,
                lex_string_literal,
        )),
        skip_ws
    ))(i) {
        Ok((rem, (_, token, _))) => Ok((rem, token)),
        Err(err) => Err(err)
    }
}

pub fn lex_typedef(i: &str) -> nom::IResult<&str, Token> {
    nom::combinator::value(Token::TypeDefintion, nom::character::complete::char(':'))(i)
}

pub fn skip_ws(i: &str) -> nom::IResult<&str, &str> {
    nom::character::complete::space0(i)
}

pub fn skip_nl(i: &str) -> nom::IResult<&str, &str> {
    nom::character::complete::line_ending(i)
}

pub fn lex_identifier(i: &str) -> nom::IResult<&str, Token> {
    match nom::character::complete::alpha1(i) {
        Ok((rem, ident)) => Ok((rem, Token::Identifier(ident.to_string()))),
        Err(e) => Err(e)
    }
}

pub fn lex_assign(i: &str) -> nom::IResult<&str, Token> {
    nom::combinator::value(Token::Assign, nom::bytes::complete::tag("="))(i)

}

pub fn lex_arrow(i: &str) -> nom::IResult<&str, Token> {
    nom::combinator::value(Token::Arrow, nom::bytes::complete::tag("->"))(i)
}

pub fn lex_unbound(i: &str) -> nom::IResult<&str, Token> {
    nom::combinator::value(Token::Unbound, nom::bytes::complete::tag("_"))(i)
}

pub fn lex_period(i: &str) -> nom::IResult<&str, Token> {
    nom::combinator::value(Token::Period, nom::bytes::complete::tag("."))(i)
}

pub fn lex_string_literal(i: &str) -> nom::IResult<&str, Token> {
    match nom::sequence::tuple((
            nom::bytes::complete::tag("\""),
            nom::bytes::complete::take_until("\""),
            nom::bytes::complete::tag("\""),
    ))(i) {
        Ok((rem, (_, s,_))) => Ok((rem, Token::StringLiteral(s.to_string()))),
        Err(e) => Err(e)
    }
}
*/

#[derive(Debug, Clone)]
pub enum Token {
    Assign,
    Identifier(String),
    TypeDefintion,
    Arrow,
    Unbound,
    Period,
    StringLiteral(String)
}

