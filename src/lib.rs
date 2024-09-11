mod command;

use command::{Command, CommandErr};
use std::error::Error;
use std::env;
use std::{io, process};
use std::path::Path;

pub struct Config {
    command: Command,
    args: String,
}

impl Config {
    pub fn build() -> Result<Config, Box<dyn Error>> {
        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let (cmd, args) = input.split_once(char::is_whitespace).unwrap();
        let command = Self::parse_input(cmd)?;
        Ok(Config {
            command,
            args: String::from(args.trim()),
        })
    }

    fn parse_input(input: &str) -> Result<Command, CommandErr> {
        let command;
        match input {
            "exit" => {
                command = Command::Exit;
            }
            "echo" => {
                command = Command::Echo;
            }
            "type" => {
                command = Command::Type;
            }
            "pwd" => {
                command = Command::Pwd;
            }
            "cd" => {
                command = Command::CD;
            }
            other => {
                let cmd = input;
                let path_env = env::var("PATH").unwrap();

                let split = &mut path_env.split(":");
                if let Some(path) = split.find(|path| std::fs::metadata(format!("{}/{}", path, cmd)).is_ok()) {
                    command = Command::BuiltIn(String::from(cmd), format!("{path}/{cmd}"))
                } else {
                    return Err(CommandErr::CommandNotFound(String::from(other)))
                }
            },
        }
        Ok(command)
    }

    pub fn run(&self) {
        match &self.command {
            Command::Exit => {
                let status_code = match self.args.parse::<i32>() {
                    Ok(sc) => sc,
                    Err(_) => {
                        println!("Invalid status_code");
                        return;
                    }
                };
                process::exit(status_code);
            }
            Command::Echo => {
                println!("{}", self.args)
            }
            Command::Type => {
                match Self::parse_input(&self.args[..]) {
                    Ok(x) => {
                        match x {
                            Command::BuiltIn(cmd, path) => println!("{cmd} is {path}"),
                            _ => println!("{} is a shell builtin", self.args),
                        }
                    }
                    Err(_) => {
                        println!("{}: command not found", self.args)
                    }
                }
            },
            Command::Pwd => {
                let current_dir = env::current_dir().unwrap();
                println!("{}", current_dir.display());
            }
            Command::CD => {
                if env::set_current_dir(Path::new(&self.args)).is_err() {
                    println!("cd: {}: No such file or directory", self.args)
                }
            }
            Command::BuiltIn(cmd, path) => {
                process::Command::new(path)
                    .arg(&self.args)
                    .output()
                    .expect("Failed to execute command");
            }
        }
    }
}
