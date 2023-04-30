pub mod ast;
pub mod lexer;
pub mod parser;
pub mod repl;
pub mod token;

use lexer::Lexer;
use token::TokenType;

fn main() {
    loop {
        let user_input = repl::start_repl().unwrap();
        let mut lexer = Lexer::new(user_input);

        loop {
            let token = lexer.next_token();
            if token.r#type == TokenType::EOF {
                break;
            } else {
                println!("{:?}", token);
            }
        }
    }
}
