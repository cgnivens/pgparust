#![allow(missing_docs, dead_code, unused_imports)]

#[macro_use]
mod macros;
mod parser;
mod lexer;
mod reserved;
mod codemap;
pub use self::parser::Parser;

pub fn main() {
    println!("Hello world");
}
