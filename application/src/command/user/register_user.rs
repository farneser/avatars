use crate::command::{Command, CommandHandler, ValidationException};
use std::error::Error;

pub struct Register;

pub struct RegisterUserCommand {
    username: String,
    password: String,
}

impl RegisterUserCommand {
    pub fn new(username: String, password: String) -> RegisterUserCommand {
        RegisterUserCommand { username, password }
    }
}

impl Command<i64> for RegisterUserCommand {}

pub struct RegisterUserCommandHandler;

impl RegisterUserCommandHandler {
    pub fn new() -> Self {
        Self {}
    }
}

impl CommandHandler<RegisterUserCommand, i64> for RegisterUserCommandHandler {
    fn handle(&self, command: RegisterUserCommand) -> Result<i64, Box<dyn Error>> {
        if command.username.is_empty() {
            return Err(Box::new(ValidationException));
        }

        if command.password.len() < 8 {
            return Err(Box::new(ValidationException));
        }

        Ok(1)
    }
}