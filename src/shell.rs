use std::{collections::HashMap, io::{self, Write}};

use crate::{command::Command, command_handler::CommandHandler};

pub struct Shell {
    pub commands: HashMap<String, CommandHandler>,
    paths: Vec<String>,
}

impl Shell {
    pub fn new() -> Shell {

        let paths = std::env::var("PATH");

        if paths.is_err() {
            return Shell {
                commands: HashMap::new(),
                paths: Vec::new()
            };
        }

        let paths = paths.unwrap();
        let paths: Vec<String> = paths.split(":").map(|s| s.to_string()).collect();

        Shell {
            commands: HashMap::new(),
            paths
        }
    }

    pub fn add_command(&mut self, command: Command) {
        self.commands.insert(command.name, command.handler);
    }

    pub fn get_system_handler(&self, command: &str) -> Option<String> {
        self.paths.iter().find(|path| {
            let path = format!("{}/{}", path, command);
            std::path::Path::new(&path).exists()
        }).map(|s| s.to_string())
    }

    pub fn run(&self) {

        let not_found = Command::new("command_not_found", |args, _| {
            println!("{}: command not found", args.get(0).unwrap_or(&String::from("error")));
        });

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
            let command = parts.get(0);

            if command.is_none() {
                (not_found.handler)(&parts, &self);
                continue;
            }

            let command = command.unwrap();

            // Find the command
            let handler = self.commands.get(command.as_str());

            if handler.is_some() {
                let handler = handler.unwrap();
                handler(&parts, &self);
                continue;
            }

            // Get a system handler
            let system_handler = self.get_system_handler(command.as_str());

            if system_handler.is_some() {
                let system_handler = system_handler.unwrap();
                let output = std::process::Command::new(format!("{}/{}", system_handler, command))
                    .args(parts.iter().skip(1))
                    .output()
                    .expect("failed to execute process");
                print!("{}", String::from_utf8_lossy(&output.stdout));
                continue;
            }

            // If the command is not found, run the default command_not_found
            if handler.is_none() {
                (not_found.handler)(&parts, &self);
                continue;
            }
        }
    }

}
