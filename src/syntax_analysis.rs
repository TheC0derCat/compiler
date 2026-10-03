use crate::lexical_analysis::Token;

#[derive(Debug, PartialEq)]
pub enum AST {
    Root(Vec<AST>),
    Identifier(String),
    FunctionCall(String, Vec<AST>),
    // Litteral Values
    NumericLitteral(f64),
    StringLitteral(String),
    True,
    False,
    // Operators
    // Numeric
    Add(Box<AST>, Box<AST>),
    Subtract(Box<AST>, Box<AST>),
    Multiply(Box<AST>, Box<AST>),
    Divide(Box<AST>, Box<AST>),
    // Boolean
    And(Box<AST>, Box<AST>),
    Or(Box<AST>, Box<AST>),
    Not(Box<AST>),
    // Statements
    If(Box<AST>, Vec<AST>),
    While(Box<AST>, Vec<AST>),
    Definition(String, Vec<AST>),
}

pub fn parser(code: Vec<Token>) -> AST {
    todo!()
}
