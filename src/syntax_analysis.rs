use crate::lexical_analysis::*;

#[derive(Debug, PartialEq)]
pub enum DataType {
    IntType,
    StrType,
    BoolType,
}

#[derive(Debug, PartialEq)]
pub enum AST {
    Root(Vec<AST>),
    Identifier(String),
    FunctionCall(String, Vec<AST>),
    VariableDeclaration(String, DataType, Box<AST>),
    VariableAssignment(String, Box<AST>),
    // Litteral Values
    NumericLitteral(i64),
    StringLitteral(String),
    True,
    False,
    // Operators
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

pub fn parse_expr(expr: Vec<Token>) -> AST {
    // Variable Assignment and Declaration
    let mut i: usize = 0;
    while i < expr.len() {
        if expr[i] == Token::SetTo {
            match expr[i - 1] {
                // Assignment
                Token::Identifier(ref s) => {
                    return AST::VariableAssignment(
                        s.to_string(),
                        Box::new(parse_expr(expr[(i + 1)..expr.len()].to_vec())),
                    );
                }
                // Declaration
                _ => {
                    let thetype: DataType = match expr[i - 1] {
                        Token::IntType => DataType::IntType,
                        Token::StrType => DataType::StrType,
                        Token::BoolType => DataType::BoolType,
                        _ => panic!("{:?} is not a valid type", expr[i - 1]),
                    };
                    if expr[i - 2] != Token::Star {
                        panic!("invalid assignment");
                    }
                    if let Token::Identifier(ref s) = expr[i - 3] {
                        return AST::VariableDeclaration(
                            s.to_string(),
                            thetype,
                            Box::new(parse_expr(expr[(i + 1)..expr.len()].to_vec())),
                        );
                    } else {
                        panic!("cant declare variable without a name for it lmao");
                    }
                }
            }
        }
        i += 1;
    }
    // Logical And and Logical Or
    let mut i: usize = 0;
    while i < expr.len() {
        if expr[i] == Token::And {
            return AST::And(
                Box::new(parse_expr(expr[0..i].to_vec())),
                Box::new(parse_expr(expr[(i + 1)..expr.len()].to_vec())),
            );
        }
        if expr[i] == Token::Or {
            return AST::Or(
                Box::new(parse_expr(expr[0..i].to_vec())),
                Box::new(parse_expr(expr[(i + 1)..expr.len()].to_vec())),
            );
        }
        i += 1;
    }
    // logical not
    let mut i: usize = 0;
    while i < expr.len() {
        if expr[i] == Token::Not {
            return AST::Not(
                Box::new(parse_expr(expr[(i + 1)..expr.len()].to_vec())),
            );
        }
        i += 1;
    }
    // Addition and Subtraction
    let mut i: usize = 0;
    while i < expr.len() {
        if expr[i] == Token::Add {
            return AST::Add(
                Box::new(parse_expr(expr[0..i].to_vec())),
                Box::new(parse_expr(expr[(i + 1)..expr.len()].to_vec())),
            );
        }
        if expr[i] == Token::Subtract {
            return AST::Subtract(
                Box::new(parse_expr(expr[0..i].to_vec())),
                Box::new(parse_expr(expr[(i + 1)..expr.len()].to_vec())),
            );
        }
        i += 1;
    }
    // Multiplication and Division
    let mut i: usize = 0;
    while i < expr.len() {
        if expr[i] == Token::Multiply {
            return AST::Multiply(
                Box::new(parse_expr(expr[0..i].to_vec())),
                Box::new(parse_expr(expr[(i + 1)..expr.len()].to_vec())),
            );
        }
        if expr[i] == Token::Divide {
            return AST::Divide(
                Box::new(parse_expr(expr[0..i].to_vec())),
                Box::new(parse_expr(expr[(i + 1)..expr.len()].to_vec())),
            );
        }
        i += 1;
    }
    // single bits
    match &expr[0] {
        Token::True => AST::True,
        Token::False => AST::False,
        Token::NumericLitteral(i) => AST::NumericLitteral(*i),
        Token::StringLitteral(i) => AST::StringLitteral(i.to_string()),
        Token::Identifier(i) => AST::Identifier(i.to_string()),
        _ => panic!("syntax error in expression parsing"),
    }
}

pub fn parse_block(block: Vec<Token>, current_indentation: u16) -> Vec<AST> {
    todo!()
}

pub fn parse(code: Vec<Token>) -> AST {
    AST::Root(parse_block(code, 0))
}
