use std::str::{FromStr};
use std::io;
use crate::reserved;

fn take_while<F>(data: &str, mut pred: F) -> io::Result<(&str, usize)>
where F: FnMut(char) -> bool
{
    let mut current_index = 0;

    for ch in data.chars() {
        let should_continue = pred(ch);

        if !should_continue {
            break;
        }

        current_index += ch.len_utf8();
    }

    if current_index == 0 {
        Err(io::Error::new(io::ErrorKind::Other, "No Matches"))
    } else {
        Ok((&data[..current_index], current_index))
    }
}

pub enum Token {
    Integer(usize),
    Decimal(f64),
    Identifier(String),
    Reserved(String),
    Function(String),
    QuotedString(String),
    Asterisk,
    At,
    Carat,
    CloseParen,
    CloseSquare,
    Colon,
    Dot,
    End,
    Equals,
    Minus,
    OpenParen,
    OpenSquare,
    Plus,
    Semicolon,
    Slash,
}

impl From<String> for Token {
    fn from(other: String) -> Token {
        Token::Identifier(other)
    }
}

impl<'a> From<&'a str> for Token {
    fn from(other: &'a str) -> Token {
        Token::Identifier(other.to_string())
    }
}

impl From<usize> for Token {
    fn from(other: usize) -> Token {
        Token::Integer(other)
    }
}

impl From<f64> for Token {
    fn from(other: f64) -> Token {
        Token::Decimal(other)
    }
}


fn tokenize_ident(data: &str) -> io::Result<(Token, usize)> {
    // Cannot start identifiers with a number
    match data.chars().next() {
        Some(ch) if ch.is_digit(10) => return Err(io::Error::new(io::ErrorKind::Other, format!("Identifiers can't start with a number"))),
        None => return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Unexpected end of stream")),
        _ => {},
    }

    let (got, bytes_read) = take_while(data, |ch| (ch == '_') || (ch.is_alphanumeric()))?;

    // match keywords here
    let tok = {
        if reserved::is_reserved(got.to_string()) {
            Token::Reserved(got.to_string())
        } else if reserved::is_function(got.to_string()){
            Token::Function(got.to_string())
        } else {
            Token::Identifier(got.to_string())
        }
    };

    Ok((tok, bytes_read))
}

enum QueryType {
    CREATE,
    SELECT,
    INSERT,
    UPDATE,
    DELETE,
}

impl FromStr for QueryType {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.to_uppercase().as_str() { // not case-sensitive, so force case
            "CREATE" => Ok(QueryType::CREATE),
            "SELECT" => Ok(QueryType::SELECT),
            "INSERT" => Ok(QueryType::INSERT),
            "UPDATE" => Ok(QueryType::UPDATE),
            "DELETE" => Ok(QueryType::DELETE),
            _ => Err("Unknown query type".into())
        }
    }
}


struct Condition {}

struct Query {
    query_type: QueryType,
    table_name: String,
    conditions: Vec<Condition>
}
