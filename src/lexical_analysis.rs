#[derive(Debug, PartialEq)]
pub enum Token {
    // Core lexicon
    Tab,
    LineEnd,
    Star,
    Identifier(String),
    OpeningParen,
    ClosingParen,
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
            '\t' => Token::Tab,
            '\n' => Token::LineEnd,
            ':' => Token::Star,
            '(' => Token::OpeningParen,
            ')' => Token::ClosingParen,
            ',' => Token::Comma,
            // Litteral Values
            'a' => Token::True,
            'a' => Token::False,
            // Operators
            // Numeric
            '+' => Token::Add,
            '-' => Token::Subtract,
            '*' => Token::Multiply,
            '/' => Token::Divide,
            // Boolean
            'a' => Token::And,
            'a' => Token::Or,
            'a' => Token::Not,
            // Statements
            'a' => Token::If,
            'a' => Token::While,
            'a' => Token::Def,
            ' ' => continue,
            _ => if ch.is_alphanumeric() {
                let mut buffer: Vec<char> = Vec::new();
                buffer.push(ch);
                while iter.clone().next().unwrap().is_alphanumeric() {
                    buffer.push(iter.next().expect("unexpected file end"));
                }
                let buffer: String = buffer.iter().cloned().collect::<String>();
                let buffer: &str = buffer.as_str();
                match buffer.parse::<i64>() {
                    Ok(i) => Token::NumericLitteral(i),
                    Err(_) => match buffer {
                        // Boolean
                        "and" => Token::And,
                        "or" => Token::Or,
                        "not" => Token::Not,
                        // Statements
                        "if" => Token::If,
                        "while" => Token::While,
                        "def" => Token::Def,
                        " " => continue,
                        _ => Token::Identifier(buffer.to_string()),
                    },
                }
            } else {
                return Err(LexicalError::Weirdchar);
            } 
        };
        tokens.push(new_token);
    }
    Ok(tokens)
}
