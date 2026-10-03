pub mod lexical_analysis;
pub mod syntax_analysis;
use crate::lexical_analysis::*;
use crate::syntax_analysis::*;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_lexical_analysis() {
        assert_eq!(lex("".to_string()), Err(LexicalError::EmptyString));
        assert_eq!(
            lex("1+1\n".to_string()),
            Ok(vec![
                Token::NumericLitteral(1),
                Token::Add,
                Token::NumericLitteral(1)
            ])
        );
        assert_eq!(
            lex("1 + 1\n".to_string()),
            Ok(vec![
                Token::NumericLitteral(1),
                Token::Add,
                Token::NumericLitteral(1)
            ])
        );assert_eq!(
            lex("print(\"Hello, world!\")\n".to_string()),
            Ok(vec![
                Token::Identifier("print".to_string()),
                Token::OpeningParen,
                Token::StringLitteral("Hello, world!".to_string()),
                Token::ClosingParen,
            ])
        );
    }
}
