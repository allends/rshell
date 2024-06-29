use std::collections::HashMap;
#[allow(unused_imports)]
use std::io::{self, Write};

struct Command {
    name: String,
    handler: fn(&Vec<String>),
}

impl Command {
    fn new(name: &str, handler: fn(&Vec<String>)) -> Command {
        Command {
            name: name.to_string(),
            handler: handler,
        }
    }

    fn run(&self, args: &Vec<String>) {
        (self.handler)(args);
    }
}

fn main() {
    // Uncomment this block to pass the first stage
    print!("$ ");
    io::stdout().flush().unwrap();

    // Define the commands

    // Default command not found
    let command_not_found = Command::new("command_not_found", |args| {
        println!("Command not found: {}", args[0]);
    });

    let mut commands = HashMap::new();

    commands.insert("echo", Command::new("echo", |args| {
        println!("{}", args[1]);
    }));


    // Wait for user input
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();

    // Parse the input
    let parts: Vec<String> = input.trim().split_whitespace().map(|str| { str.to_string()}).collect();
    let command = parts.get(0).unwrap();

    // Find the command
    let handler = commands.get(command.as_str()).unwrap_or(&command_not_found);

    // Run the command
    handler.run(&parts);
}
