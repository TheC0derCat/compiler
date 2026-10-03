#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // Core lexicon
    Tab,
    LineEnd,
    Star,
    Identifier(String),
    OpeningParen,
    ClosingParen,
    SetTo,
    Comma,
    // Litteral Values
    NumericLitteral(i64),
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
    // Types
    IntType,
    StrType,
}
#[derive(Debug, PartialEq)]
pub enum LexicalError {
    EmptyString,
    Weirdchar,
}
pub fn lex(code: String) -> Result<Vec<Token>, LexicalError> {
    if code.is_empty() {
        return Err(LexicalError::EmptyString);
    }
    let mut iter = code.chars();
    let mut tokens: Vec<Token> = Vec::new();
    while let Some(ch) = iter.next() {
        let new_token = match ch {
            // Core lexicon
            '\t' => Token::Tab,
            '\n' => Token::LineEnd,
            ':' => Token::Star,
            '(' => Token::OpeningParen,
            ')' => Token::ClosingParen,
            '=' => Token::SetTo,
            ',' => Token::Comma,
            // Operators
            '+' => Token::Add,
            '-' => Token::Subtract,
            '*' => Token::Multiply,
            '/' => Token::Divide,
            // Other
            ' ' => continue,
            '"' => {
                let mut buffer: String = String::new();
                while iter.clone().next().unwrap() != '"' {
                    buffer.push(iter.next().expect("unexpected file end"));
                }
                iter.next();
                Token::StringLitteral(buffer)
            }
            _ => {
                if ch.is_alphanumeric() {
                    let mut buffer: String = String::new();
                    buffer.push(ch);
                    while iter.clone().next().unwrap().is_alphanumeric() {
                        buffer.push(iter.next().expect("unexpected file end"));
                    }
                    let buffer: &str = buffer.as_str();
                    match buffer.parse::<i64>() {
                        Ok(i) => Token::NumericLitteral(i),
                        Err(_) => match buffer {
                            // Boolean
                            "true" => Token::True,
                            "false" => Token::False,
                            "and" => Token::And,
                            "or" => Token::Or,
                            "not" => Token::Not,
                            // Statements
                            "if" => Token::If,
                            "while" => Token::While,
                            "def" => Token::Def,
                            "int" => Token::IntType,
                            "str" => Token::StrType,
                            _ => Token::Identifier(buffer.to_string()),
                        },
                    }
                } else {
                    return Err(LexicalError::Weirdchar);
                }
            }
        };
        tokens.push(new_token);
    }
    Ok(tokens)
}
