#![deny(dead_code)]
#![deny(missing_docs)]

//! The Lua library.
//! Contains the full lexer, parser, vm and runtime.

// Lexer
mod token;
pub mod lexer;

// Parser
pub mod parser;

#[cfg(test)]
mod tests {
    use lexer::{Lexer, Lexeme};
    use token::Token;
    use std::iter::Iterator;
    macro_rules! matchseq {
        ($lex:expr$(,$a:expr)*) => {{
            let mut lex = &mut $lex as &mut Lexer;
            $({
                match Iterator::next(lex) {
                    Some(Lexeme(tk, _)) => assert_eq!(tk, Token::from($a)),
                    None => unimplemented!(),
                };
            })* {
                while Iterator::next(lex).is_some() {}
            }
        }};
    }
    #[test]
    fn lex_op_dot() {
        let src = format!(". .. ...");
        let mut lex = Lexer::new(&src);
        matchseq!(lex, Token::MemberAccess, Token::Concat, Token::VarArgs);
    }
    #[test]
    fn lex_op_comp() {
        let src = format!("< <= > >= == ~=");
        let mut lex = Lexer::new(&src);
        matchseq!(lex,
                  Token::LessThan,
                  Token::LessThanEqual,
                  Token::GreaterThan,
                  Token::GreaterThanEqual,
                  Token::Equal,
                  Token::NotEqual);
    }
    #[test]
    fn lex_comment() {
        let src = format!("\n-- hello, world!\n");
        let mut lex = Lexer::new(&src);
        matchseq!(lex, Token::Comment(format!("hello, world!")));
    }
    #[test]
    fn lex_hashbang() {
        let src = format!("#!/usr/bin/env lua\n");
        let mut lex = Lexer::new(&src);
        matchseq!(lex, Token::Hashbang(format!("/usr/bin/env lua")));
    }
    #[test]
    #[should_panic]
    fn lex_hashbang_invalid() {
        let src = format!("\n#!/usr/bin/env lua\n");
        let mut lex = Lexer::new(&src);
        matchseq!(lex);
    }
    #[test]
    fn lex_ident() {
        let src = format!("hello world");
        let mut lex = Lexer::new(&src);
        matchseq!(lex, "hello", "world");
    }
    #[test]
    fn lex_num_dec_int() {
        let src = format!("1234");
        let mut lex = Lexer::new(&src);
        matchseq!(lex, Token::Number(1234f64));
    }
    #[test]
    fn lex_num_dec_int_exp() {
        let src = format!("1234E31");
        let mut lex = Lexer::new(&src);
        matchseq!(lex, Token::Number(1234E31f64));
    }
    #[test]
    #[should_panic]
    fn lex_num_dec_int_invalid_exp() {
        let src = format!("1234EFF");
        let mut lex = Lexer::new(&src);
        matchseq!(lex);
    }
    #[test]
    fn lex_num_dec_float() {
        let src = format!("1.234");
        let mut lex = Lexer::new(&src);
        matchseq!(lex, Token::Number(1.234f64));
    }
    #[test]
    fn lex_num_hex_int() {
        let src = format!("0xFFFF");
        let mut lex = Lexer::new(&src);
        matchseq!(lex, Token::Number(65535f64));
    }
    #[test]
    #[should_panic]
    fn lex_num_hex_eof() {
        let src = format!("0x");
        let mut lex = Lexer::new(&src);
        matchseq!(lex);
    }
    #[test]
    #[should_panic]
    fn lex_num_hex_misformed() {
        let src = format!("0xy");
        let mut lex = Lexer::new(&src);
        matchseq!(lex);
    }
    #[test]
    fn lex_str() {
        let src = format!("\"Hello, '\\\"world!\\\"'\"\n'ayoo\\a'");
        let mut lex = Lexer::new(&src);
        matchseq!(lex,
                  Token::StaticString(format!("Hello, '\"world!\"'")),
                  Token::StaticString(format!("ayoo\x07")));
    }
    #[test]
    fn lex_general() {
        let src = format!("function Memoize(fn) fn = fn or function(x) return nil end return \
                           setmetatable({{}}, {{ __index = function(t, k) local val = fn(k) t[k] \
                           = val return val end, __call  = function(t, k) return t[k] end }}) end");
        let mut lex = Lexer::new(&src);
        matchseq!(lex,
                  "function",
                  "Memoize",
                  Token::OpenParen,
                  "fn",
                  Token::CloseParen,
                  "fn",
                  Token::Assignment,
                  "fn",
                  "or",
                  "function",
                  Token::OpenParen,
                  "x",
                  Token::CloseParen,
                  "return",
                  "nil",
                  "end",
                  "return",
                  "setmetatable",
                  Token::OpenParen,
                  Token::OpenBrace,
                  Token::CloseBrace,
                  Token::Comma,
                  Token::OpenBrace,
                  "__index",
                  Token::Assignment,
                  "function",
                  Token::OpenParen,
                  "t",
                  Token::Comma,
                  "k",
                  Token::CloseParen,
                  "local",
                  "val",
                  Token::Assignment,
                  "fn",
                  Token::OpenParen,
                  "k",
                  Token::CloseParen,
                  "t",
                  Token::OpenBracket,
                  "k",
                  Token::CloseBracket,
                  Token::Assignment,
                  "val",
                  "return",
                  "val",
                  "end",
                  Token::Comma,
                  "__call",
                  Token::Assignment,
                  "function",
                  Token::OpenParen,
                  "t",
                  Token::Comma,
                  "k",
                  Token::CloseParen,
                  "return",
                  "t",
                  Token::OpenBracket,
                  "k",
                  Token::CloseBracket,
                  "end",
                  Token::CloseBrace,
                  Token::CloseParen,
                  "end");
    }
}
