#![allow(dead_code)]

//! The semantic analyser.
//! Performs semantic analysis on a set of lexical tokens.

mod ast {
    //! The abstract syntax tree.

    /// Container type for holding statements.
    pub struct Container(Vec<Stmt>);

    /// A statement.
    pub enum Stmt {
        Expr(Expr),
    }

    /// An expression.
    pub enum Expr {
        Call(Name, Vec<Box<Expr>>),
    }

    pub struct Name(String);
}

use lexer::Lexeme;
use parser::ast::*;

/// AstVisitor trait.
/// Provides visitors for the AST.
#[allow(missing_docs)]
trait AstVisitor {
    fn visit_name(&mut self, val: &Name);
}

/// Semantic analyser.
pub struct Parser {
    tokens: Box<Vec<Lexeme>>,
}

#[allow(unused_variables)]
impl AstVisitor for Parser {
    fn visit_name(&mut self, val: &Name) {
        unimplemented!()
    }
}

/// Implements `Parser`.
impl Parser {
    /// Construct a new `Parser`.
    pub fn new(tokens: Vec<Lexeme>) -> Parser {
        Parser { tokens: Box::new(tokens) }
    }
    /// Analyse a set of lexical tokens.
    pub fn parse() -> Container {
        unimplemented!()
    }
}