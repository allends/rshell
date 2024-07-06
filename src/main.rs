use std::collections::HashMap;
#[allow(unused_imports)]
use std::io::{self, Write};

struct Shell {
    commands: HashMap<String, Handler>,
}

impl Shell {
    fn new() -> Shell {
        Shell {
            commands: HashMap::new(),
        }
    }

    fn add_command(&mut self, command: Command) {
        self.commands.insert(command.name, command.handler);
    }

    fn run(&self) {

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

            // If the command is not found, run the default command_not_found
            if handler.is_none() {
                (not_found.handler)(&parts, &self);
                continue;
            }

            let handler = handler.unwrap();

            // Run the command
            handler(&parts, &self);
        }
    }

}

type Handler = fn(&Vec<String>, &Shell);

#[derive(Debug, Clone)]
struct Command {
    name: String,
    handler: Handler,
}

impl Command {
    fn new(name: &str, handler: Handler) -> Command {
        Command {
            name: name.to_string(),
            handler,
        }
    }
}

fn main() {
    // Define the commands
    let mut shell = Shell::new();

    // Empty command
    shell.add_command(Command::new("", |_, _| {}));

    shell.add_command(Command::new("echo", |args, _| {
        let rest = args.iter().skip(1).fold(String::new(), |acc, arg| acc + " " + arg);
        println!("{}", rest.trim());
    }));

    shell.add_command(Command::new("exit", |args, _| {
        let status = args.get(1).unwrap_or(&String::from("0")).parse::<i32>().unwrap_or(0);
        std::process::exit(status);
    }));

    shell.add_command(Command::new("type", |args, shell| {
        let command = args.get(1);
        if command.is_none() {
            println!("type: missing argument");
            return;
        }
        let command = command.unwrap();
        if shell.commands.contains_key(command.as_str()) {
            println!("{} is a shell builtin", command);
            return;
        }
        println!("{}: not found", command);
    }));

    // Run the shell
    shell.run();
}
