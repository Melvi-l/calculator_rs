use std::cmp::Ordering;

#[derive(Debug, PartialEq)]
pub enum Operator {
    ADD,
    SUB,
    MULT,
    DIV,
}

impl PartialOrd for Operator {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        use Operator::*;
        match (self, other) {
            (ADD, MULT) | (ADD, DIV) => Some(Ordering::Less),
            (SUB, MULT) | (SUB, DIV) => Some(Ordering::Less),
            (MULT, ADD) | (MULT, SUB) => Some(Ordering::Greater),
            (DIV, ADD) | (DIV, SUB) => Some(Ordering::Greater),
            (_, _) => Some(Ordering::Equal),
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Token {
    EOF,
    Number(f32),
    Operator(Operator),
    LPAREN,
    RPAREN,
}

pub struct Calculator {}

#[derive(Debug, PartialEq)]
pub enum LexerError {
    BadToken(char),
}

pub struct Lexer {
    position: usize,
    read_position: usize,
    ch: u8,
    input: Vec<u8>,
}

impl Lexer {
    pub fn from(input: &str) -> Self {
        let mut lexer = Self {
            position: 0,
            read_position: 0,
            ch: 0,
            input: input.as_bytes().to_vec()
        };
        lexer.read();
        lexer
    }
    pub fn read(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input[self.read_position]
        }
        self.position = self.read_position;
        self.read_position += 1;
    }
    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read();
        }
    }
    pub fn get_token(&mut self) -> Result<Token, LexerError> {
        self.skip_whitespace();

        let result = match self.ch {
            b'0'..=b'9' => return Ok(Token::Number(self.read_number())),
            b'+' => Ok(Token::Operator(Operator::ADD)),
            b'-' => Ok(Token::Operator(Operator::SUB)),
            b'*' => Ok(Token::Operator(Operator::MULT)),
            b'/' => Ok(Token::Operator(Operator::DIV)),
            b'(' => Ok(Token::LPAREN),
            b')' => Ok(Token::RPAREN),
            0 => Ok(Token::EOF),
            bad_byte => Err(LexerError::BadToken(bad_byte as char)),
        };

        self.read();

        result
    }
    fn read_number(&mut self) -> f32 {
        let pos = self.position;
        while self.ch.is_ascii_digit() || self.ch == b'.' {
            self.read();
        }
        let number_str = String::from_utf8_lossy(&self.input[pos..self.position]);
        number_str.parse::<f32>().unwrap()
    }
}

#[test]
pub fn test_operator() {
    let input = "1+2-3*4/5";
    let expected_token_list = vec![
        Token::Number(1.),
        Token::Operator(Operator::ADD),
        Token::Number(2.),
        Token::Operator(Operator::SUB),
        Token::Number(3.),
        Token::Operator(Operator::MULT),
        Token::Number(4.),
        Token::Operator(Operator::DIV),
        Token::Number(5.),
    ];
    let mut lexer = Lexer::from(input);
    for expected_token in expected_token_list {
        println!("{:?}", expected_token);
        let next_token = lexer.get_token();
        assert_eq!(next_token.unwrap(), expected_token);
    }
}

#[test]
pub fn test_float() {
    let input = "1.302+2.456";
    let expected_token_list = vec![
        Token::Number(1.302),
        Token::Operator(Operator::ADD),
        Token::Number(2.456),
    ];
    let mut lexer = Lexer::from(input);
    for expected_token in expected_token_list {
        println!("{:?}", expected_token);
        let next_token = lexer.get_token();
        assert_eq!(next_token.unwrap(), expected_token);
    }
}
