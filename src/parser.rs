#![allow(dead_code)]

//! The semantic analyser.
//! Performs semantic analysis on a set of lexical tokens.

mod ast {
    //! The abstract syntax tree.

    /// Pseudo type for holding statements.
    pub struct Block(pub Vec<Stmt>);

    /// Implements `Block`.
    impl Block {
        pub fn add_child(&mut self, stmt: Stmt) {
            self.0.push(stmt);
        }
    }

    /// A name for something.
    pub struct Name(pub String);

    /// A statement.
    pub enum Stmt {
        Do(Block),
        /// # EBNF
        /// ```plain
        /// set_stmt = name {"," name} "=" expr {"," expr}
        /// ```
        /// # Examples
        /// ```lua
        /// a = 0
        /// b, c = a, a
        /// ```
        Set(Vec<(Name, Expr)>),
        /// # EBNF
        /// ```plain
        /// while_stmt = "while" expr "do" block "end"
        /// ```
        While(Expr, Block),
        /// # EBNF
        /// ```plain
        /// repeat_stmt = "repeat" block "until" expr
        /// ```
        Repeat(Expr, Block),
        /// # EBNF
        /// ```plain
        /// if_stmt = "if" expr "then" block {"else if" expr "then" block} ["else" block] end
        /// ```
        If(Vec<(Expr, Block)>, Option<Block>),
        /// # EBNF
        /// ```plain
        /// for_num_stmt = "for" name "=" expr "," expr ["," expr] "do" block "end"
        /// ```
        ForNum(Name, Expr, Expr, Option<Expr>, Block),
        /// # EBNF
        /// ```plain
        /// for_in_stmt = "for" name {"," name} "in" expr {"," expr} "do" block "end"
        /// ```
        ForIn(Vec<(Name, Expr)>, Block),
        /// # EBNF
        /// ```plain
        /// local_stmt = "local" name {"," name} "=" expr {"," expr}
        /// ```
        Local(Vec<(Name, Expr)>),
        /// # EBNF
        /// ```plain
        /// goto_stmt = "goto" string
        /// ```
        Goto(String),
        /// # EBNF
        /// ```plain
        /// label_stmt = "::" string "::"
        /// ```
        Label(String),
        /// # EBNF
        /// ```plain
        /// return_stmt = "return" expr {"," expr}
        /// ```
        Return(Vec<Expr>),
        /// # EBNF
        /// ```plain
        /// break_stmt = "break"
        /// ```
        Break,
    }

    /// An expression.
    pub enum Expr {
        Nil,
        Dots,
        True,
        False,
        Number(f64),
        StaticString(String),
        Call(Name, Vec<Box<Expr>>),
    }
}

use lexer::Lexeme;
use parser::ast::*;

/// AstVisitor trait.
/// Provides visitors for the AST.
#[allow(missing_docs)]
trait AstVisitor {
    fn visit_name(&mut self, val: &Name);
}

/// Cursor.
struct Cursor {
    pos: usize,
    max_pos: usize,
}

macro_rules! min {
    ($a:expr,$b:expr) => {{
        let a = $a;
        let b = $b;
        if a < b { a } else { b }
    }}
}

/// Implements `Cursor`.
impl Cursor {
    /// Constructs a new `Cursor`.
    fn new(max_pos: usize) -> Cursor {
        Cursor {
            pos: 0,
            max_pos: max_pos,
        }
    }
    #[inline(always)]
    fn next(&mut self) {
        self.pos = min!(self.pos + 1, self.max_pos);
    }
    #[inline(always)]
    fn skip(&mut self, n: usize) {
        self.pos = min!(self.pos + n, self.max_pos);
    }
}

/// Parsing unit.
struct ParsingUnit<'a, T: 'a> {
    pos: usize,
    size: usize,
    tokens: &'a [T],
}

/// Implements `ParsingUnit`.
impl<'a, T> ParsingUnit<'a, T> {
    /// Constructs a new `ParsingUnit`.
    fn new(items: &'a [T]) -> ParsingUnit<'a, T> {
        ParsingUnit {
            pos: 0,
            size: items.len(),
            tokens: items,
        }
    }
    fn see(self, lookahead: usize) -> bool {
        self.pos + lookahead < self.size
    }
}

/// Semantic analyser.
pub struct Parser<'a> {
    src: ParsingUnit<'a, Lexeme>,
}

/// Implements `Parser`.
impl<'a> Parser<'a> {
    /// Constructs a new `Parser`.
    pub fn new(tokens: &'a Vec<Lexeme>) -> Parser<'a> {
        Parser { src: ParsingUnit::new(tokens) }
    }
    /// Analyses the semantics of a set of lexical tokens.
    pub fn parse(&mut self) -> Block {
        macro_rules! see {
            () => (see!(1usize));
            ($lookahead:expr) => (self.src.pos + usize::from($lookahead) < self.src.size);
        }
        let mut root = Block(vec![]);
        while see!() {
            root.add_child({
                // Statement
                unimplemented!()
            });
        }
        root
    }
}

#[allow(unused_variables)]
impl<'a> AstVisitor for Parser<'a> {
    fn visit_name(&mut self, val: &Name) {
        unimplemented!()
    }
}