use std::str::{FromStr};
use std::io;
use std::error;
use std::fmt;
use std::num::{ParseIntError, ParseFloatError};
use serde::{Serialize, Deserialize};
use crate::reserved;
use crate::codemap::{Span};

#[derive(Debug)]
pub enum LexerError {
    BadNumber(String),
    IOError(String),
    UnknownCharacter(String),
    EOFError(String),
    MessageWithLocation(String),
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
                Self::MessageWithLocation(err_desc) => err_desc,
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TokenKind {
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
    Comma,
}

impl From<String> for TokenKind {
    fn from(other: String) -> TokenKind {
        if reserved::is_function(&other){
            TokenKind::Function(other)
        } else if reserved::is_reserved(&other) {
            TokenKind::Reserved(other)
        } else {
            TokenKind::Identifier(other)
        }
    }
}

impl FromStr for TokenKind {
    // Wrap in the checks for functions/reserved keywords
    // here to make it easier
    type Err = io::Error;
    fn from_str(other: &str) -> io::Result<TokenKind> {
        if reserved::is_function(other){
            Ok(TokenKind::Function(other.to_string()))
        } else if reserved::is_reserved(other) {
            Ok(TokenKind::Reserved(other.to_string()))
        } else {
            Ok(TokenKind::Identifier(other.to_string()))
        }
    }
}

impl From<usize> for TokenKind {
    fn from(other: usize) -> TokenKind {
        TokenKind::Integer(other)
    }
}

impl From<f64> for TokenKind {
    fn from(other: f64) -> TokenKind {
        TokenKind::Decimal(other)
    }
}

impl From<&str> for TokenKind {
    fn from(other: &str) -> TokenKind {
        if reserved::is_function(other){
            TokenKind::Function(other.to_string())
        } else if reserved::is_reserved(other) {
            TokenKind::Reserved(other.to_string())
        } else {
            TokenKind::Identifier(other.to_string())
        }
    }
}


fn tokenize_ident(data: &str) -> io::Result<(TokenKind, usize)> {
    // Cannot start identifiers with a number
    match data.chars().next() {
        Some(ch) if ch.is_digit(10) => return Err(io::Error::new(io::ErrorKind::Other, format!("Identifiers can't start with a number"))),
        None => return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Unexpected end of stream")),
        _ => {},
    }

    let (got, bytes_read) = take_while(data, |ch| (ch == '_') || (ch.is_alphanumeric()))?;

    // match keywords using from_str implementation
    let tok = TokenKind::from_str(got)?;

    Ok((tok, bytes_read))
}


fn tokenize_number(data: &str) -> Result<(TokenKind, usize), LexerError> {
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
        Ok((TokenKind::Decimal(n), bytes_read))
    } else {
        let n: usize = decimal.parse()?;
        Ok((TokenKind::Integer(n), bytes_read))
    }
}

fn tokenize_quote_string(data: &str) -> Result<(TokenKind, usize), LexerError> {
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

    Ok((TokenKind::QuotedString(s.to_string()), bytes_read))
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

fn skip(src: &str) -> usize {
    // Generic skip function wrapping the whitespace and comments
    // logic
    let mut remaining = src;

    loop {
        let ws = skip_whitespace(remaining);
        remaining = &remaining[ws..];
        let comments = skip_comments(remaining);
        remaining = &remaining[comments..];

        if ws + comments == 0 {
            return src.len() - remaining.len();
        }
    }
}

pub fn tokenize_single_token(data: &str) -> Result<(TokenKind, usize), LexerError> {
    let next = match data.chars().next() {
        Some(c) => c,
        None => return Err(LexerError::EOFError("Hit end of file".to_string())),
    };

    let (tok, length) = match next {
        '.' => (TokenKind::Dot, 1),
        '=' => (TokenKind::Equals, 1),
        '+' => (TokenKind::Plus, 1),
        '-' => (TokenKind::Minus, 1),
        '*' => (TokenKind::Asterisk, 1),
        '/' => (TokenKind::Slash, 1),
        '@' => (TokenKind::At, 1),
        '^' => (TokenKind::Carat, 1),
        '(' => (TokenKind::OpenParen, 1),
        ')' => (TokenKind::CloseParen, 1),
        '[' => (TokenKind::OpenSquare, 1),
        ']' => (TokenKind::CloseSquare, 1),
        ',' => (TokenKind::Comma, 1),
        '0' ... '9' => tokenize_number(data)?,
        '"' => tokenize_quote_string(data)?,
        c @ '_' | c if c.is_alphabetic() => tokenize_ident(data)?,
        other => return Err(LexerError::UnknownCharacter(format!("Hit unknown character {:?}", other))),
    };

    Ok((tok, length))
}

struct Tokenizer<'a> {
    current_index: usize,
    remaining_text: &'a str,
}

impl<'a> Tokenizer<'a> {
    fn new(src: &str) -> Tokenizer {
        Tokenizer {
            current_index: 0,
            remaining_text: src,
        }
    }

    fn next_token(&mut self) -> Result<Option<(TokenKind, usize, usize)>, LexerError> {
        self.skip_whitespace();

        if self.remaining_text.is_empty() {
            Ok(None)
        } else {
            let start = self.current_index;
            let tok = match self._next_token() {
                Ok(t) => t,
                _ => return Err(LexerError::MessageWithLocation(
                        format!("Couldn't read next token at {}", self.current_index)
                    ))
            };
            let end = self.current_index;
            Ok(Some((tok, start, end)))
        }
    }

    fn skip_whitespace(&mut self) {
        let skipped = skip(self.remaining_text);
        self.chomp(skipped);
    }

    fn _next_token(&mut self) -> Result<TokenKind, LexerError> {
        let (tok, bytes_read) = tokenize_single_token(self.remaining_text)?;
        self.chomp(bytes_read);

        Ok(tok)
    }

    fn chomp(&mut self, num_bytes: usize) {
        self.remaining_text = &self.remaining_text[num_bytes..];
        self.current_index += num_bytes;
    }
}


pub fn tokenize(src: &str) -> Result<Vec<(TokenKind, usize, usize)>, LexerError> {
    let mut tokenizer = Tokenizer::new(src);
    let mut tokens = Vec::new();

    while let Some(tok) = tokenizer.next_token()? {
        tokens.push(tok);
    }

    Ok(tokens)
}


pub struct Token {
    pub span: Span,
    pub kind: TokenKind,
}


impl Token {
    pub fn new<K: Into<TokenKind>>(span: Span, kind: K) -> Token {
        let kind = kind.into();
        Token { span, kind }
    }
}


impl<T> From<T> for Token
where T: Into<TokenKind> {
    fn from(other: T) -> Token {
        Token::new(Span::dummy(), other)
    }
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

#[cfg(test)]
#[test]
fn tokenize_basic_select() {
    let src = "SELECT * FROM mytable";
    let should_be: Vec<(TokenKind, usize, usize)> = vec![
        (TokenKind::from_str("SELECT").unwrap(), 0, 6),
        (TokenKind::Asterisk, 7, 8),
        (TokenKind::from_str("FROM").unwrap(), 9, 13),
        (TokenKind::from_str("mytable").unwrap(), 14, 21),
    ];
    let tokens = tokenize(src).unwrap();

    assert_eq!(tokens, should_be);
}

#[cfg(test)]
#[test]
fn broken_snippet() {
    let src = "SELECT \"broken FROM mytable";
    let err = tokenize(src);
    assert_eq!(err.is_err(), true);
    let e = err.unwrap_err();

    match e {
        LexerError::MessageWithLocation(s) => assert_eq!(s, "Couldn't read next token at 7"),
        other => panic!("Unexpected error: {}", other),
    }
}


#[cfg(test)]
#[test]
fn test_builtins() {
    let src = "SELECT COUNT(1) FROM mytable";
    let should_be: Vec<(TokenKind, usize, usize)> = vec![
        (TokenKind::Reserved("SELECT".to_string()), 0, 6),
        (TokenKind::Function("COUNT".to_string()), 7, 12),
        (TokenKind::OpenParen, 12, 13),
        (TokenKind::from(1), 13, 14),
        (TokenKind::CloseParen, 14, 15),
        (TokenKind::Reserved("FROM".to_string()), 16, 20),
        (TokenKind::from_str("mytable").unwrap(), 21, 28)
    ];
    let tokens = tokenize(src).unwrap();
    assert_eq!(tokens, should_be);
}

#[cfg(test)]
#[test]
fn test_alias() {
    let src = "SELECT t.hello FROM mytable t";
    let should_be: Vec<(TokenKind, usize, usize)> = vec![
        (TokenKind::Reserved("SELECT".to_string()), 0, 6),
        (TokenKind::from_str("t").unwrap(), 7, 8),
        (TokenKind::Dot, 8, 9),
        (TokenKind::from_str("hello").unwrap(), 9, 14),
        (TokenKind::Reserved("FROM".to_string()), 15, 19),
        (TokenKind::from_str("mytable").unwrap(), 20, 27),
        (TokenKind::from_str("t").unwrap(), 28, 29),
    ];
    let tokens = tokenize(src).unwrap();
    assert_eq!(tokens, should_be);
}


#[cfg(test)]
#[test]
fn test_left_join() {
    let src = "LEFT JOIN abc d ON d.id = t.id";
    let should_be: Vec<(TokenKind, usize, usize)> = vec![
        (TokenKind::from_str("LEFT").unwrap(), 0, 4),
        (TokenKind::from_str("JOIN").unwrap(), 5, 9),
        (TokenKind::from_str("abc").unwrap(), 10, 13),
        (TokenKind::from_str("d").unwrap(), 14, 15),
        (TokenKind::Reserved("ON".to_string()), 16, 18),
        (TokenKind::from_str("d").unwrap(), 19, 20),
        (TokenKind::Dot, 20, 21),
        (TokenKind::from_str("id").unwrap(), 21, 23),
        (TokenKind::Equals, 24, 25),
        (TokenKind::from_str("t").unwrap(), 26, 27),
        (TokenKind::Dot, 27, 28),
        (TokenKind::from_str("id").unwrap(), 28, 30),
    ];
    let tokens = tokenize(src).unwrap();
    assert_eq!(tokens, should_be);
}
