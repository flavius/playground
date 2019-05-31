use super::logging;
use crate::Plugin;
use crate::application::{Command, AsCommand, AsAny};

use std::collections::HashMap;
use std::convert::From;
use std::rc::Rc;

use super::super::application;
use super::super::application::command;

enum CommandName {
    Me,
    Help,
    ImplicitHelp,
    NewTask,
}

impl From<&str> for CommandName {
    fn from(value: &str) -> Self {
        use self::CommandName::*;
        match value {
            "me" => Me,
            "help" => Help,
            "new" => NewTask,
            _ => ImplicitHelp,
        }
    }
}

pub struct Cli {
    logger: Box<dyn logging::LogWriter>,
    args: Vec<String>,
    env: HashMap<String, String>,
    original_args: Vec<String>,
    command_bus: command::CommandBus,
}

impl Cli {
    pub fn new(mut args: Vec<String>, env: HashMap<String, String>, logging: &logging::Logging) -> Self {
        let ctx = logging.new_context("cli".to_owned());
        let logger = Box::new(logging.new_logger(ctx));
        let original_args = args.clone();
        let command_bus = command::CommandBus::new();
        Self {
            logger,
            args,
            env,
            original_args,
            command_bus,
        }
    }

    fn get_command(&mut self, command_name: CommandName) -> Rc<dyn Command> {
        use CommandName::*;
        let command = match command_name {
            Me => {
                let contact = self.args.remove(0);
                let nickname = self.args.remove(0);
                command::Me::new(contact, nickname).as_command()
            },
            NewTask => {
                let description = self.args.clone();
                command::NewTask::new(description).as_command()
            },
            Help => command::Help::new(false, self.original_args.clone()).as_command(),
            ImplicitHelp => command::Help::new(true, self.original_args.clone()).as_command(),
        };
        command
    }
}

impl Plugin for Cli {
    fn run(&mut self) {
        self.logger.log_raw("run".to_owned());
        let command_name : CommandName = CommandName::from(self.args.remove(0).as_str());
        let mut command : Rc<dyn Command> = self.get_command(command_name);
        self.command_bus.executeCommand(command);
    }
    fn shutdown(&mut self) {
        self.logger.log_raw("shutdown".to_owned());
    }
}

