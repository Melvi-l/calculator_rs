use evaluator::Evaluator;
use lexer::Lexer;
use parser::Parser;
use wasm_bindgen::prelude::*;
mod lexer;
mod parser;
mod evaluator;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn compute(input: &str) {
    let mut parser = Parser::from(input);
    let rpn_token_list = match parser.parse() {
        Ok(result) => result,
        Err(err) => {
            alert(&format!("Parsing Error: {:?}", err));
            return;
        }
    };
    let result = Evaluator::evaluate(rpn_token_list);
    match result {
        Ok(number) => alert(&format!("Great, here is your result: {}!", number)),
        Err(err) => alert(&format!("Evaluation Error: {:?}", err)),
    }
}

