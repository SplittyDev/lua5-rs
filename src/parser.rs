#![allow(dead_code)]

//! The semantic analyser.
//! Performs semantic analysis on a set of lexical tokens.

mod ast {
    //! The abstract syntax tree.

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

use parser::ast::*;

/// AstVisitor trait.
/// Provides visitors for the AST.
#[allow(missing_docs)]
trait AstVisitor {
    fn visit_name(&mut self, val: &Name);
}

struct Parser;

#[allow(unused_variables)]
impl AstVisitor for Parser {
    fn visit_name(&mut self, val: &Name) {
        unimplemented!()
    }
}

impl Parser {
    pub fn parse() -> Stmt {
        unimplemented!()
    }
}