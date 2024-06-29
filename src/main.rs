use std::collections::HashMap;
#[allow(unused_imports)]
use std::io::{self, Write};

#[derive(Debug, Clone)]
struct Command {
    name: String,
    handler: fn(&Vec<String>),
}

impl Command {
    fn new(name: &str, handler: fn(&Vec<String>)) -> Command {
        Command {
            name: name.to_string(),
            handler,
        }
    }

    fn run(&self, args: &Vec<String>) {
        (self.handler)(args);
    }
}

fn main() {
    // Define the commands

    // Empty command
    let empty_command = Command::new("", |_| {
    });

    // Default command not found
    let command_not_found = Command::new("command_not_found", |args| {
        println!("{}: command not found", args.get(0).unwrap_or(&String::from("error")));
    });

    let mut commands: HashMap<&str, Command> = HashMap::new();

    commands.insert("echo", Command::new("echo", |args| {
        println!("{}", args[1]);
    }));

    commands.insert("exit", Command::new("exit", |args| {
        let status = args.get(1).unwrap_or(&String::from("0")).parse::<i32>().unwrap_or(0);
        std::process::exit(status);
    }));

    commands.insert(&command_not_found.name, command_not_found.clone());
    commands.insert(&empty_command.name, empty_command.clone());

    loop {
        //  Print the prompt
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
    
        // Parse the input
        let parts: Vec<String> = input.trim().split_whitespace().map(|str| { str.to_string()}).collect();
        let command = parts.get(0).unwrap_or(&String::from("")).clone();
    
        // Find the command
        let handler = commands.get(command.as_str()).unwrap_or(&command_not_found);
    
        // Run the command
        handler.run(&parts);
    }
}
