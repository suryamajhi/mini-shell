mod command;

use mini_shell::Config;
use std::io::{self, Write};

fn main() {
    loop {
        // Uncomment this block to pass the first stage
        print!("$ ");
        io::stdout().flush().unwrap();

        let config = match Config::build() {
            Ok(command) => command,
            Err(err) => {
                println!("{}", err);
                continue;
            }
        };

        config.run();
    }
}
