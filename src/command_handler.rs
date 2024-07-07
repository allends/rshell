use crate::shell::Shell;

pub type CommandHandler = fn(&Vec<String>, &Shell);
