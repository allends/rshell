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

        let system_handler = shell.get_system_handler(command);

        if system_handler.is_some() {
            let system_handler = system_handler.unwrap();
            println!("{} is {}/{}", command, system_handler, command);
            return;
        }
        println!("{}: not found", command);
    }));

    shell.add_command(Command::new("pwd", |_, _| {
        let path = std::env::current_dir().unwrap();
        println!("{}", path.display());
    }));

    shell.add_command(Command::new("cd", |args, _| {
        let path = args.get(1);
        if path.is_none() {
            println!("cd: missing argument");
            return;
        }
        let path = path.unwrap();

        if path == "~" {
            let home = std::env::var("HOME");
            if home.is_err() {
                println!("cd: HOME not set");
                return;
            }
            let home = home.unwrap();
            let result = std::env::set_current_dir(&home);
            if result.is_err() {
                println!("cd: {}: No such file or directory", home);
            }
            return;
        }

        let result = std::env::set_current_dir(path);
        if result.is_err() {
            println!("cd: {}: No such file or directory", path);
        }
    }));

    // Run the shell
    shell.run();
}
