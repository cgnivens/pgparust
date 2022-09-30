/// Shorthand macro for generating a token from *anything* which can be
/// converted into a `TokenKind`, or any of the `TokenKind` variants.
///
/// # Examples
///
/// ```
/// #[macro_use]
/// extern crate static_analyser;
///
/// # fn main() {
/// tok!(Dot);
/// tok!(123);
/// tok!(3.14);
/// tok!(OpenParen);
/// # }
/// ```
#[macro_export]
macro_rules! tok {
    ($thing:tt) =>  {
        {
            #[allow(unused_imports)]
            use $crate::lexer::TokenKind::*;
            $crate::lexer::Token::from($thing)
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::codemap::Span;
    use crate::lexer::{Token, TokenKind};

    macro_rules! token_macro_test {
        ($name:ident, $from:tt => $to:expr) => {
            #[test]
            fn $name() {
                let got: Token = tok!($from);
                let should_be = Token::new(Span::dummy(), $to);

                assert_eq!(got, should_be);
            }
        }
    }

    token_macro_test!(tok_expands_to_dot, Dot => TokenKind::Dot);
    token_macro_test!(tok_expands_to_openparen, OpenParen => TokenKind::OpenParen);
    token_macro_test!(tok_expands_to_integer, 1234 => TokenKind::Integer(1234));
    token_macro_test!(tok_expands_to_decimal, 12.34 => TokenKind::Decimal(12.34));
    token_macro_test!(tok_expands_to_identifier, "Hello" => TokenKind::Identifier("Hello".to_string()));
    token_macro_test!(tok_expands_to_reserved, "SELECT" => TokenKind::Reserved("SELECT".to_string()));
}
