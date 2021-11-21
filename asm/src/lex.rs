use std::error::Error;
use std::fmt::{self, Display};
use std::result;

use lazy_static::lazy_static;
use regex::Regex;

use crate::uarch;

type Result<T> = result::Result<T, LexemeError>;

pub fn tokenize(line: &str) -> Option<Vec<String>> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\b").unwrap();
    }
    let tokens: Vec<_> = line
        .split_whitespace() // split at whitespace
        .flat_map(|s| RE.split(s).filter(|s| !s.is_empty()))
        .map(String::from) // convert from &str -> String
        .take_while(|s| !s.eq(";")) // strip comments
        .collect();
    match tokens.len() {
        0 => None,
        _ => Some(tokens),
    }
}

pub fn parse_reg(token: &str) -> Result<uarch> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^r(\d+)$").unwrap();
    }
    (|| match token {
        "sp" => Some(13),
        "lr" => Some(14),
        "pc" => Some(15),
        token => uarch::from_str_radix(RE.captures(token)?.get(1)?.as_str(), 10).ok(),
    })()
    .ok_or_else(|| LexemeError::InvalidReg(token.to_string()))
}

pub fn parse_imm(token: &str) -> Result<uarch> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^0(b|d|o|x)([[:xdigit:]]+)$").unwrap();
    }
    (|| {
        let captures = RE.captures(token)?;
        match captures.get(1)?.as_str() {
            "b" => uarch::from_str_radix(captures.get(2)?.as_str(), 2).ok(),
            "d" => uarch::from_str_radix(captures.get(2)?.as_str(), 10).ok(),
            "o" => uarch::from_str_radix(captures.get(2)?.as_str(), 8).ok(),
            "x" => uarch::from_str_radix(captures.get(2)?.as_str(), 16).ok(),
            _ => None,
        }
    })()
    .ok_or_else(|| LexemeError::InvalidImm(token.to_string()))
}

#[derive(Debug)]
pub enum LexemeError {
    EmptyToken,
    InvalidReg(String),
    InvalidImm(String),
}

impl Display for LexemeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::EmptyToken => "Found empty token; expected content".to_string(),
                Self::InvalidReg(token) => format!("Could not parse register from `{}`", token),
                Self::InvalidImm(token) => format!("Could not parse immediate from `{}`", token),
            }
        )
    }
}

impl Error for LexemeError {}
