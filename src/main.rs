#[allow(unused_imports)]
use std::io::{self, Write};

use command::Command;
use shell::Shell;

mod shell;
mod command;
mod command_handler;

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
        if shell.get_system_handler(command).is_some() {
            println!("{} is a system command", command);
            return;
        }
        println!("{}: not found", command);
    }));

    // Run the shell
    shell.run();
}
