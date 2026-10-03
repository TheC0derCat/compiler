pub enum Token {
    // Core syntax
    Tab,
    LineEnd,
    Star,
    Identifier,
    OpeningParen,
    ClosingParen,
    Comma,
    // Litteral Values
    NumericLitteral(f64),
    StringLitteral(String),
    True,
    False,
    // Operators
    // Numeric
    Add,
    Subtract,
    Multiply,
    Divide,
    // Boolean
    And,
    Or,
    Not,
    // Statements
    If,
    While,
    Def,
}
fn Lexer(code: String) -> Option<Vec<Token>> {
    todo!()
}
