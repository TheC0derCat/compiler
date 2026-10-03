use crate::lexical_analysis;

pub enum AST {
	Root(Vec<AST>),
	Identifier(String),
	// Litteral Values
	NumericLitteral(f64),
	StringLitteral(String),
	True,
	False,
	// Operators
	// Numeric
	Add(AST, AST),
	Subtract(AST, AST),
	Multiply(AST, AST),
	Divide(AST, AST),
	// Boolean
	And(AST, AST),
	Or(AST, AST),
	Not(AST),
	// Statements
	If(AST, Vec<AST>),
	While(AST, Vec<AST>),
	Def(String, Vec<AST>),
}