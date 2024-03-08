use crate::lexer::{Operator, Token};

#[derive(Debug, PartialEq)]
pub enum EvaluatorError {
    NotSingleResultInStack,
    MissingOperand,
}

pub struct Evaluator {}
impl Evaluator {
    pub fn evaluate(mut rpn_token_list: Vec<Token>) -> Result<f32, EvaluatorError> {
        rpn_token_list.reverse();
        let mut stack = Vec::new();
        while let Some(token) = rpn_token_list.pop() {
            match token {
                Token::Number(n) => stack.push(n as f32),
                Token::Operator(operator) => {
                    let right = match stack.pop() {
                        Some(n) => n,
                        None => return Err(EvaluatorError::MissingOperand),
                    };
                    let left = match stack.pop() {
                        Some(n) => n,
                        None => return Err(EvaluatorError::MissingOperand),
                    };
                    match operator {
                        Operator::ADD => stack.push(left + right),
                        Operator::SUB => stack.push(left - right),
                        Operator::MULT => stack.push(left * right),
                        Operator::DIV => stack.push(left / right),
                    }
                }
                _ => panic!("what's dat"),
            }
        }

        if stack.len() > 1 {
            return Err(EvaluatorError::NotSingleResultInStack);
        }
        match stack.pop() {
            Some(number) => Ok(number),
            None => Ok(0.0),
        }
    }
}

#[test]
fn simple_add() {
    let rpn_token_list = vec![
        Token::Number(5.),
        Token::Number(5.),
        Token::Operator(Operator::ADD),
    ];
    let expected = 10.0;
    let actual = Evaluator::evaluate(rpn_token_list).unwrap();
    assert_eq!(actual, expected)
}
#[test]
fn simple_sub() {
    let rpn_token_list = vec![
        Token::Number(25.),
        Token::Number(5.),
        Token::Operator(Operator::SUB),
    ];
    let expected = 20.0;
    let actual = Evaluator::evaluate(rpn_token_list).unwrap();
    assert_eq!(actual, expected)
}
#[test]
fn simple_multiply() {
    let rpn_token_list = vec![
        Token::Number(5.),
        Token::Number(5.),
        Token::Operator(Operator::MULT),
    ];
    let expected = 25.0;
    let actual = Evaluator::evaluate(rpn_token_list).unwrap();
    assert_eq!(actual, expected)
}
#[test]
fn simple_divide() {
    let rpn_token_list = vec![
        Token::Number(25.),
        Token::Number(5.),
        Token::Operator(Operator::DIV),
    ];
    let expected = 5.0;
    let actual = Evaluator::evaluate(rpn_token_list).unwrap();
    assert_eq!(actual, expected)
}
#[test]
fn full() {
    let rpn_token_list = vec![
        Token::Number(3.),
        Token::Number(4.),
        Token::Operator(Operator::ADD),
        Token::Number(10.),
        Token::Operator(Operator::MULT),
        Token::Number(35.),
        Token::Operator(Operator::DIV),
    ];
    let expected = 2.0;
    let actual = Evaluator::evaluate(rpn_token_list).unwrap();
    assert_eq!(actual, expected)
}

// Error
#[test]
fn missing_left_operand() {
    let rpn_token_list = vec![Token::Number(4.), Token::Operator(Operator::ADD)];
    let result = Evaluator::evaluate(rpn_token_list);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), EvaluatorError::MissingOperand);
}
#[test]
fn missing_right_operand() {
    let rpn_token_list = vec![Token::Operator(Operator::ADD)];
    let result = Evaluator::evaluate(rpn_token_list);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), EvaluatorError::MissingOperand);
}
