use crate::command_handler::CommandHandler;

#[derive(Debug, Clone)]
pub struct Command {
    pub name: String,
    pub handler: CommandHandler,
}

impl Command {
    pub fn new(name: &str, handler: CommandHandler) -> Command {
        Command {
            name: name.to_string(),
            handler,
        }
    }
}