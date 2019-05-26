use super::logging;
use crate::Plugin;
use crate::application::Command;

use std::collections::HashMap;
use std::convert::From;
use std::rc::Rc;

use super::super::application;
use super::super::application::command;
use super::super::application::AsCommand;

enum CommandName {
    Me,
    Help,
    ImplicitHelp,
}

impl From<&str> for CommandName {
    fn from(value: &str) -> Self {
        use self::CommandName::*;
        match value {
            "me" => Me,
            "help" => Help,
            _ => ImplicitHelp,
        }
    }
}

pub struct Cli {
    logger: Box<dyn logging::LogWriter>,
    args: Vec<String>,
    env: HashMap<String, String>,
}

impl Cli {
    pub fn new(mut args: Vec<String>, env: HashMap<String, String>, logging: &logging::Logging) -> Self {
        let ctx = logging.new_context("cli".to_owned());
        let logger = Box::new(logging.new_logger(ctx));
        Self {
            logger,
            args,
            env,
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
            Help => command::Help::new(false, self.args.clone()).as_command(),
            ImplicitHelp => command::Help::new(true, self.args.clone()).as_command(),
        };
        command
    }
}

impl Plugin for Cli {
    fn run(&mut self) {
        self.logger.log_raw("run".to_owned());
        let command_name : CommandName = CommandName::from(self.args.remove(0).as_str());
        let mut command : Rc<dyn Command> = self.get_command(command_name);
        Rc::get_mut(&mut command).unwrap().execute();
    }
    fn shutdown(&mut self) {
        self.logger.log_raw("shutdown".to_owned());
    }
}

