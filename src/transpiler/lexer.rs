extern crate regex;

use regex::Regex;

pub use self::Token::{
    Def,
    If,
    Then,
    Else,
    For,
    In,
    Var,
    Num,
    Str,
    Delimiter,
    OpeningParenthesis,
    ClosingParenthesis,
    Comma,
    Ident,
    Number,
    Operator
};

#[derive(PartialEq, Clone, Debug)]
pub enum Token {
    Def,
    If,
    Then,
    Else,
    For,
    In,
    Var,
    Num,
    Str,
    Delimiter,      // ';' character
    OpeningParenthesis,
    ClosingParenthesis,
    Comma,
    Ident(String),
    Number(f64),
    Operator(String)
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let comment_re = Regex::new(r"(?m)#.*\n").unwrap();
    let preprocessed = comment_re.replace_all(input, "\n");

    let mut result = Vec::new();

    let token_re = Regex::new(concat!(
        r"(?P<ident>\p{Alphabetic}\w*)|",
        r"(?P<number>\d+\.?\d*)|",
        r"(?P<delimiter>;)|",
        r"(?P<oppar>\()|",
        r"(?P<clpar>\))|",
        r"(?P<comma>,)|",
        r"(?P<operator>\S)")).unwrap();

    for cap in token_re.captures_iter(&preprocessed.to_string()) {
        let token = if cap.name("ident").is_some() {
            match cap.name("ident").unwrap().as_str() {
                "def" => Def,
                "if" => If,
                "then" => Then,
                "else" => Else,
                "for" => For,
                "in" => In,
                "var" => Var,
                "num" => Num,
                "str" => Str,
                ident => Ident(ident.to_string())
            }
        } else if cap.name("number").is_some() {
            match cap.name("number").unwrap().as_str().parse() {
                Ok(number) => Number(number),
                Err(_) => panic!("Lexer failed trying to parse number")
            }
        } else if cap.name("delimiter").is_some() {
            Delimiter
        } else if cap.name("oppar").is_some() {
            OpeningParenthesis
        } else if cap.name("clpar").is_some() {
            ClosingParenthesis
        } else if cap.name("comma").is_some() {
            Comma
        } else {
            Operator(cap.name("operator").unwrap().as_str().to_string())
        };

        result.push(token)
    }

    result
}
