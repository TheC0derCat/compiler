pub enum Token {
	Tab,
	LineEnd,
	Star,
	Identifier,
	OpeningParen,
	ClosingParen,
	Comma,
	NumericLitteral(f64),
	Add,
	Subtract,
	Multiply,
	Divide,
}

pub struct Lexer {
}
impl Iterator for Lexer {
	type Item = Token;
}