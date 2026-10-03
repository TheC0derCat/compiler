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
            lex("1+1 ".to_string()),
            Ok(vec![
                Token::NumericLitteral(1),
                Token::Add,
                Token::NumericLitteral(1),
            ])
        );
        assert_eq!(
            lex(" 1 + 1 ".to_string()),
            Ok(vec![
                Token::NumericLitteral(1),
                Token::Add,
                Token::NumericLitteral(1)
            ])
        );
        assert_eq!(
            lex("print(\"Hello, world!\") ".to_string()),
            Ok(vec![
                Token::Identifier("print".to_string()),
                Token::OpeningParen,
                Token::StringLitteral("Hello, world!".to_string()),
                Token::ClosingParen,
            ])
        );
        assert_eq!(
            lex("x: int = 20\n".to_string()),
            Ok(vec![
                Token::Identifier("x".to_string()),
                Token::Star,
                Token::IntType,
                Token::SetTo,
                Token::NumericLitteral(20),
                Token::LineEnd,
            ])
        );
        assert_eq!(
            lex("x#hi\n3 ".to_string()),
            Ok(vec![
                Token::Identifier("x".to_string()),
                Token::LineEnd,
                Token::NumericLitteral(3),
            ])
        );
    }
    #[test]
    fn test_syntax_analysis() {
        assert_eq!(
            parse_expr(vec![
                Token::NumericLitteral(1),
                Token::Add,
                Token::NumericLitteral(1)
            ]),
            AST::Add(
                Box::new(AST::NumericLitteral(1)),
                Box::new(AST::NumericLitteral(1))
            )
        );
        assert_eq!(
            parse_expr(vec![
                Token::NumericLitteral(1),
                Token::Add,
                Token::NumericLitteral(1),
                Token::Multiply,
                Token::NumericLitteral(1),
            ]),
            AST::Add(
                Box::new(AST::NumericLitteral(1)),
                Box::new(AST::Multiply(
                    Box::new(AST::NumericLitteral(1)),
                    Box::new(AST::NumericLitteral(1))
                )),
            )
        );
        assert_eq!(
            parse_expr(vec![
                Token::Identifier("myvar".to_string()),
                Token::SetTo,
                Token::NumericLitteral(17),
                Token::Add,
                Token::NumericLitteral(42)
            ]),
            AST::VariableAssignment(
                "myvar".to_string(),
                Box::new(AST::Add(
                    Box::new(AST::NumericLitteral(17)),
                    Box::new(AST::NumericLitteral(42))
                ))
            )
        );
        assert_eq!(
            parse_expr(vec![
                Token::Identifier("myvar".to_string()),
                Token::Star,
                Token::IntType,
                Token::SetTo,
                Token::NumericLitteral(500)
            ]),
            AST::VariableDeclaration(
                "myvar".to_string(),
                DataType::IntType,
                Box::new(AST::NumericLitteral(500))
            )
        );
    }
}
