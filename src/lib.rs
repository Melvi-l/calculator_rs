mod evaluator;
mod lexer;
mod parser;

use evaluator::Evaluator;
use lexer::Lexer;
use parser::Parser;

pub fn calc(input: &str) -> String {
    let mut parser = Parser::from(input);
    match parser.parse() {
        Ok(rpn_token_list) => {
            match Evaluator::evaluate(rpn_token_list) {
                Ok(number) => format!("{:.2}", number),
                Err(err) => format!("{:?}", err)
            }
        },
        Err(err) => format!("{:?}", err)
    }
}
