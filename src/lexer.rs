#![deny(dead_code)]
#![deny(missing_docs)]

//! The lexical analyser.
//! Performs lexical analysis on Lua source code.

use std::fmt;
use std::str::Chars;
use std::iter::Peekable;
use token::Token;

/// A lexical token with positional information.
pub struct Lexeme(pub Token, pub TokenPosition);

/// Positional information for lexical tokens.
#[derive(Debug, Clone, Copy)]
pub struct TokenPosition {
    /// The current line.
    line: u32,
    /// The position on the current line.
    pos: u32,
}

/// Implements `Default` for `TokenPosition`.
impl Default for TokenPosition {
    fn default() -> TokenPosition {
        TokenPosition { line: 1, pos: 0 }
    }
}

/// Implements `Display` for `TokenPosition`.
impl fmt::Display for TokenPosition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}]", self.line, self.pos)
    }
}

/// Lexical analyser.
#[derive(Debug, Clone)]
pub struct Lexer<'a> {
    /// The peekable buffer.
    buf: Peekable<Chars<'a>>,
    /// The current position.
    pos: TokenPosition,
}

/// Implements `Lexer`.
impl<'a> Lexer<'a> {
    /// Constructs a new `Lexer`.
    pub fn new(src: &'a String) -> Lexer<'a> {
        Lexer {
            buf: src.chars().peekable().to_owned(),
            pos: TokenPosition::default(),
        }
    }
}

/// Implements `Iterator` for `Lexer`.
impl<'a> Iterator for Lexer<'a> {
    type Item = Lexeme;

    /// Reads the next `Item`.
    fn next(&mut self) -> Option<Lexeme> {

        // The current position.
        let now: TokenPosition;

        // Can be set if skipping a character after matching is not desired.
        let mut no_skip = false;

        /// Logs a message.
        macro_rules! log {
            (INFO $msg:expr) => (println!(format!("{:?} {}", self.pos, String::from($msg))));
            (ERR $msg:expr) => (panic!(format!("{:?} {}", self.pos, String::from($msg))));
        }

        /// Peeks at a character in the stream.
        /// Currently, peeking at the next character (n=0) is way faster,
        /// because the buffer doesn't have to be cloned for that.
        macro_rules! peek {

            // Simple case, just peeks at the current character.
            () => {
                match self.buf.peek() {
                    Some(chr) => Some(chr.to_owned()),
                    _ => None,
                }
            };

            // Unfortunate case, peeks at a character more than 0 steps away.
            // Currently very expensive, optimizes to the simple case if n == 0.
            ($n:expr) => {{
                let n = $n as usize;
                if n == 0 {
                    peek!()
                } else {
                    // TODO:
                    // Find a solution to the buffer-cloning.
                    // This is supposed to be fast!
                    match self.buf.clone().skip(n).next() {
                        Some(chr) => Some(chr.to_owned()),
                        _ => None,
                    }
                }
            }};
        }

        /// Skips a specific amount of characters.
        /// This macro is to be used for the purpose of advancing the stream.
        /// It keeps track of the current line and the cursor position on the current line.
        macro_rules! skip {
            ($n:expr) => {
                for _ in 0..($n as usize) {
                    let chr: Option<char> = match peek!() {
                        Some(chr) => Some(chr.to_owned()),
                        _ => None,
                    };
                    if chr.is_some() {
                        let chr = chr.unwrap();
                        match chr {
                            '\n' => {
                                self.pos.line += 1;
                                self.pos.pos = 0;
                            }
                            _ => self.pos.pos += 1,
                        };
                        self.buf.next();
                    }
                }
            };
        }

        /// Scans an operator based on the next character in the stream.
        macro_rules! scan_op {
            ($expected:expr, $tk:expr) => {
                match peek!(1) {
                    Some(chr) if chr == ($expected as char) => {
                        skip!(1);
                        ($tk as Token)
                    }
                    Some(other) => log!(ERR format!("Unimplemented operator: `{}`", other)),
                    None => log!(ERR "Unexpected end of stream."),
                }
            };
            ($expected:expr, $tka:expr, $tkb:expr) => {
                match peek!(1) {
                    Some(chr) if chr == ($expected as char) => {
                        skip!(1);
                        ($tka as Token)
                    }
                    _ => ($tkb as Token),
                }
            };
        }

        /// Advances the stream as long as the current character is considered whitespace.
        macro_rules! skip_whitespace {
            () => {
                while peek!().is_some() && peek!().unwrap().is_whitespace() {
                    skip!(1);
                }
            }
        }

        /// Reads a line from the character stream.
        macro_rules! read_line {
            () => {{
                skip_whitespace!();
                let mut comment = String::new();
                loop {
                    match self.buf.peek().cloned() {
                        Some('\n') | None => break,
                        Some(chr) => {
                            skip!(1);
                            comment.push(chr)
                        }
                    }
                }
                comment
            }};
        }

        /// Creates a (Token, TokenPosition) tuple.
        macro_rules! emit {
            ($token:expr)
            => (Some(Lexeme($token as Token, now)));
            ($token:expr, $pos:expr)
            => (Some(Lexeme($token as Token, $pos as TokenPosition)));
        }

        /// Creates a (Token, TokenPosition) tuple using scan_op and emit.
        macro_rules! emitx {
            ($expected:expr, $tk:expr)
            => (emit!(scan_op!(($expected as char), ($tk as Token))));
            ($expected:expr, $tka:expr, $tkb:expr)
            => (emit!(scan_op!(($expected as char), ($tka as Token), ($tkb as Token))));
        }

        // Skip whitespace.
        skip_whitespace!();

        // Update the current position.
        now = self.pos;

        // The actual lexical analysis is done here.
        if let Some(chr) = peek!() {
            let result = match chr {
                '(' => emit!(Token::OpenParen),
                ')' => emit!(Token::CloseParen),
                '[' => emit!(Token::OpenBracket),
                ']' => emit!(Token::CloseBracket),
                '{' => emit!(Token::OpenBrace),
                '}' => emit!(Token::CloseBrace),
                '|' => emit!(Token::Lambda),
                ',' => emit!(Token::Comma),
                ';' => emit!(Token::Semicolon),
                '+' => emit!(Token::Add),
                '*' => emit!(Token::Mul),
                '/' => emit!(Token::Div),
                '%' => emit!(Token::Mod),
                '^' => emit!(Token::Power),
                '$' => emit!(Token::Dollar),
                '~' => emitx!('=', Token::NotEqual),
                '=' => emitx!('=', Token::Equal, Token::Assignment),
                '<' => emitx!('=', Token::LessThanEqual, Token::LessThan),
                '>' => emitx!('=', Token::GreaterThanEqual, Token::GreaterThan),
                ':' => emitx!(':', Token::DoubleColon, Token::Colon),
                '.' => {
                    match peek!(1) {
                        Some('.') => {
                            skip!(1);
                            emitx!('.', Token::VarArgs, Token::Concat)
                        }
                        _ => emit!(Token::MemberAccess),
                    }
                }
                '#' => {
                    match peek!(1) {
                        Some('!') if now.line == 1 && now.pos == 0 => {
                            skip!(2);
                            let line = read_line!();
                            emit!(Token::Hashbang(line))
                        }
                        Some('!') => {
                            log!(ERR "The shebang has to be on the first line!");
                        }
                        Some(_) | None => emit!(Token::Len),
                    }
                }
                '-' => {
                    match peek!(1) {
                        Some('-') => {
                            skip!(2);
                            let line = read_line!();
                            emit!(Token::Comment(line))
                        }
                        Some(_) | None => emit!(Token::SubOrMinus),
                    }
                }
                '"' | '\'' => {
                    let mut buf = String::new();
                    let delimiter = peek!().unwrap();
                    skip!(1);
                    while let Some(chr) = peek!() {
                        match chr {
                            '\\' => {
                                skip!(1);
                                if let Some(chr) = peek!() {
                                    skip!(1);
                                    buf.push(match chr {
                                        '\\' => '\\',
                                        '\'' => '\'',
                                        '"' => '"',
                                        'a' => '\x07',
                                        'b' => '\x08',
                                        'v' => '\x0b',
                                        'f' => '\x0c',
                                        'n' => '\n',
                                        'r' => '\r',
                                        't' => '\t',
                                        '[' => '[',
                                        ']' => ']',
                                        _ => log!(ERR format!("Invalid escape code: `\\{}`", chr)),
                                    });
                                } else {
                                    log!(ERR "Unexpected end of string.")
                                }
                            }
                            _ if chr == delimiter => {
                                skip!(1);
                                break;
                            }
                            _ => {
                                skip!(1);
                                buf.push(chr);
                            }
                        }
                    }
                    emit!(Token::StaticString(buf))
                }
                chr => {
                    if chr.is_alphabetic() || chr == '_' {
                        let mut buf = String::new();
                        loop {
                            match peek!() {
                                Some(chr) if chr.is_alphanumeric() || chr == '_' => {
                                    skip!(1);
                                    buf.push(chr);
                                }
                                Some(_) | None => {
                                    no_skip = true;
                                    break;
                                }
                            }
                        }
                        emit!(Token::Ident(buf))
                    } else if chr.is_digit(10) {
                        // The following is some EBNF I found online.
                        //
                        // INT: Digit+
                        // HEX: '0' [xX] HexDigit+
                        // FLOAT: Digit+ '.' Digit* ExponentPart?
                        // 		| '.' Digit+ ExponentPart?
                        // 		| Digit+ ExponentPart
                        // ExponentPart: [eE] [+-]? Digit+
                        //
                        // Important implementation details:
                        // - Hexadecimal floats are completely ignored here.
                        // - Hexadecimal exponents are also ignored.
                        // Reason: No sane person would ever use those.
                        // PS: Hexadecimal exponents may actually be useful.
                        // PSS: Hexadecimal floats though.. just don't.
                        let mut buf = String::new();
                        let mut has_exponent = false;
                        let mut has_fractional = false;
                        let is_hexadecimal = {
                            match peek!(1) {
                                Some(chr) if vec!['x', 'X'].contains(&chr) => {
                                    if peek!().unwrap_or(' ') != '0' {
                                        false
                                    } else {
                                        match peek!(2) {
                                            Some(chr) if !chr.is_digit(16) => {
                                                log!(ERR format!("Unexpected character in hexnum: `{}`", chr))
                                            }
                                            None => log!(ERR "Unexpected end of hexnum."),
                                            Some(_) => (),
                                        }
                                        skip!(2);
                                        true
                                    }
                                }
                                _ => false,
                            }
                        };
                        macro_rules! is_num {
                            ($chr:expr) => {{
                                let chr = $chr as char;
                                if has_exponent && !chr.is_digit(10) {
                                    log!(ERR format!("Unexpected character in exponent: `{}`", chr));
                                }
                                (is_hexadecimal && chr.is_digit(16)) ||
                                (!is_hexadecimal && chr.is_digit(10))
                            }};
                        }
                        loop {
                            match peek!() {
                                Some(chr) if chr == '.' => {
                                    if has_fractional {
                                        log!(ERR "A number can contain one fractional part at max!");
                                    } else {
                                        has_fractional = true;
                                        skip!(1);
                                        buf.push(chr);
                                    }
                                }
                                Some(chr) if is_num!(chr.to_owned()) => {
                                    skip!(1);
                                    buf.push(chr);
                                }
                                Some(chr) if vec!['e', 'E'].contains(&chr) => {
                                    if has_exponent {
                                        log!(ERR "A number can contain one exponent at max!");
                                    } else {
                                        has_exponent = true;
                                        skip!(1);
                                        buf.push(chr);
                                        match peek!() {
                                            Some('-') => {
                                                skip!(1);
                                                buf.push(chr);
                                            }
                                            Some(_) | None => (),
                                        }
                                    }
                                }
                                Some(_) | None => break,
                            }
                        }
                        if is_hexadecimal {
                            match i64::from_str_radix(buf.as_str(), 16) {
                                Ok(num) => emit!(Token::Number(num as f64)),
                                Err(msg) => log!(ERR format!("{:?}", msg)),
                            }
                        } else {
                            match buf.parse::<f64>() {
                                Ok(num) => emit!(Token::Number(num)),
                                Err(_) => log!(ERR format!("The number `{}` is malformed and doesn't parse.", buf)),
                            }
                        }
                    } else {
                        log!(ERR format!("Unimplemented operator: `{}`", chr))
                    }
                }
            };
            if !no_skip {
                skip!(1);
            }
            result
        } else {
            None
        }
    }
}