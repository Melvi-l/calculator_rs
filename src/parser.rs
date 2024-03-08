use crate::{lexer::{LexerError, Operator, Token}, Lexer};

// Hard to get a more precise error detection in the parser with the Shunting Yard Algorithm, need
// a scoped pratt parser
#[derive(Debug, PartialEq)]
pub enum ParserError {
    BadToken(char),
    MissingLeftParenthesis,
    MissingRightParenthesis,
}

pub struct Parser {
    lexer: Lexer,
}

impl Parser {
    pub fn from(input: &str) -> Self {
        let lexer = Lexer::from(input);
        Self { lexer }
    }
    pub fn parse(&mut self) -> Result<Vec<Token>, ParserError> {
        let mut queue = Vec::new();
        let mut stack: Vec<Token> = Vec::new();
        let mut current_token = match self.lexer.get_token() {
            Ok(token) => token,
            Err(LexerError::BadToken(ch)) => return Err(ParserError::BadToken(ch)),
        };
        while current_token != Token::EOF {
            match current_token {
                Token::Number(_) => queue.push(current_token),
                Token::Operator(_) => {
                    while !stack.is_empty()
                        && stack.last().unwrap() != &Token::LPAREN
                        && stack.last().unwrap() >= &current_token
                    {
                        queue.push(stack.pop().unwrap());
                    }
                    stack.push(current_token)
                }
                Token::LPAREN => stack.push(current_token),
                Token::RPAREN => {
                    while let Some(element) = stack.last() {
                        if element == &Token::LPAREN {
                            break;
                        }
                        queue.push(stack.pop().unwrap());
                    }

                    if stack.pop() != Some(Token::LPAREN) {
                        return Err(ParserError::MissingLeftParenthesis);
                    }
                }
                Token::EOF => {}
            }
            current_token = self.lexer.get_token().unwrap();
        }
        while !stack.is_empty() {
            let token_to_move = stack.pop().unwrap();
            if token_to_move == Token::LPAREN {
                return Err(ParserError::MissingRightParenthesis);
            }
            queue.push(token_to_move);
        }

        Ok(queue)
    }
}

#[test]
pub fn test_shaunting() {
    let input = "1*2+3";
    let expected_token_list = vec![
        Token::Number(1.),
        Token::Number(2.),
        Token::Operator(Operator::MULT),
        Token::Number(3.),
        Token::Operator(Operator::ADD),
    ];
    let mut parser = Parser::from(input);
    let actual_token_list = parser.parse().unwrap();
    for (expected_token, actual_token) in expected_token_list.iter().zip(actual_token_list.iter()) {
        println!("{:?}", expected_token);
        assert_eq!(actual_token, expected_token);
    }
}
#[test]
pub fn test_shaunting_paren() {
    let input = "1*(2+3)";
    let expected_token_list = vec![
        Token::Number(1.),
        Token::Number(2.),
        Token::Number(3.),
        Token::Operator(Operator::ADD),
        Token::Operator(Operator::MULT),
    ];
    let mut parser = Parser::from(input);
    let actual_token_list = parser.parse().unwrap();
    for (expected_token, actual_token) in expected_token_list.iter().zip(actual_token_list.iter()) {
        println!("{:?}", expected_token);
        assert_eq!(actual_token, expected_token);
    }
}
#[test]
pub fn test_shaunting_complex() {
    let input = "12+45/8*9";
    let expected_token_list = vec![
        Token::Number(12.),
        Token::Number(45.),
        Token::Number(8.),
        Token::Operator(Operator::DIV),
        Token::Number(9.),
        Token::Operator(Operator::MULT),
        Token::Operator(Operator::ADD),
    ];
    let mut parser = Parser::from(input);
    let actual_token_list = parser.parse().unwrap();
    for (expected_token, actual_token) in expected_token_list.iter().zip(actual_token_list.iter()) {
        println!("{:?}", expected_token);
        assert_eq!(actual_token, expected_token);
    }
}

// Test error
#[test]
fn missing_left_parenthesis() {
    let input = "1*2+3)";
    let mut parser = Parser::from(input);
    let result = parser.parse();
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), ParserError::MissingLeftParenthesis);

    let input = ")";
    let mut parser = Parser::from(input);
    let result = parser.parse();
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), ParserError::MissingLeftParenthesis);
}
#[test]
fn missing_right_parenthesis() {
    let input = "1*(2+3";
    let mut parser = Parser::from(input);
    let result = parser.parse();
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), ParserError::MissingRightParenthesis);

    let input = "(";
    let mut parser = Parser::from(input);
    let result = parser.parse();
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), ParserError::MissingRightParenthesis);
}
#[test]
fn bad_token() {
    let input = "excellent;1*(2+3";
    let mut parser = Parser::from(input);
    let result = parser.parse();
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), ParserError::BadToken('e'));
}
