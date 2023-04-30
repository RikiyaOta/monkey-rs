use std::io::{self, Write};

pub fn start_repl() -> Result<String, io::Error> {
    let mut user_input = String::new();
    print!("> ");
    io::stdout().flush()?;

    let stdin = io::stdin();
    match stdin.read_line(&mut user_input) {
        Ok(_) => Ok(user_input),
        Err(error) => Err(error)
    }
}