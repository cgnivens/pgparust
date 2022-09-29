use std::str::{FromStr};
use std::io;
use std::error;
use std::fmt;
use std::num::{ParseIntError, ParseFloatError};
use crate::reserved;

#[derive(Debug)]
pub enum LexerError {
    BadNumber(String),
    IOError(String),
    UnknownCharacter(String),
    EOFError(String),
}

impl error::Error for LexerError{}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match &*self {
                Self::UnknownCharacter(err_desc) => err_desc,
                Self::EOFError(err_desc) => err_desc,
                Self::BadNumber(err_desc) => err_desc,
                Self::IOError(err_desc) => err_desc,
            }
        )
    }
}

impl From<ParseIntError> for LexerError {
    fn from(_: ParseIntError) -> Self {
        LexerError::BadNumber(
            "Error handling conversion of integer value".to_string(),
        )
    }
}
impl From<ParseFloatError> for LexerError {
    fn from(_: std::num::ParseFloatError) -> Self {
        LexerError::BadNumber(
            "Error handling conversion of decimal value".to_string()
        )
    }
}
impl From<io::Error> for LexerError {
    fn from(_: io::Error) -> Self {
        LexerError::IOError(
            "IO Error while parsing".to_string()
        )
    }
}

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


fn tokenize_number(data: &str) -> Result<(Token, usize), LexerError> {
    let mut seen_dot = false;

    let (decimal, bytes_read) = take_while(data, |c| {
        if c.is_digit(10) {
            true
        } else if c == '.' {
            if !seen_dot {
                seen_dot = true;
                true
            } else {
                false
            }
        } else {
            false
        }
    })?;

    if seen_dot {
        let n: f64 = decimal.parse()?;
        Ok((Token::Decimal(n), bytes_read))
    } else {
        let n: usize = decimal.parse()?;
        Ok((Token::Integer(n), bytes_read))
    }
}

fn parse_quoted(data: &str) -> Result<(Token, usize), LexerError> {
    let mut seen_esc = false;

    let (s, bytes_read) = take_while(data, |c| {
        if c == '\\' {
            if !seen_esc {
                seen_esc = true;
                true
            } else {
                seen_esc = false;
                true
            }
        } else if c == '"'{
            if seen_esc {
                seen_esc = false;
                true
            } else {
                false
            }
        } else {
            true
        }
    })?;

    Ok((Token::QuotedString(s.to_string()), bytes_read))
}

fn skip_whitespace(data: &str) -> usize {
    match take_while(data, |ch| ch.is_whitespace()) {
        Ok((_, bytes_skipped)) => bytes_skipped,
        _ => 0,
    }
}

fn skip_comments(data: &str) -> usize {
    // skip both single-line and block comments
    let pairs = [("--", "\n"), ("/*", "*/")];

    for &(pattern, matcher) in &pairs {
        if data.starts_with(pattern) {
            let leftovers = skip_until(data, matcher);
            return data.len() - leftovers.len();
        }
    }

    0
}

fn skip_until<'a>(mut src: &'a str, pattern: &str) -> &'a str {
    while !src.is_empty() && !src.starts_with(pattern) {
        let next_char_size = src.chars().next().expect("The string isn't empty").len_utf8();
        src = &src[next_char_size..];
    }

    &src[pattern.len()..]
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
