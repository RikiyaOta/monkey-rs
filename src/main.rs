pub mod lexer;
pub mod token;
pub mod repl;

fn main() {
    loop {
        let user_input = repl::start_repl().unwrap();
        println!("user_input: {}", user_input);
    }
}
