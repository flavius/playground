use uuid::Uuid;
use std::fmt;
use crate::plugin;
use std::collections::HashMap;
use std::rc::Rc;

pub mod command;

pub struct Application {
    plugins: plugin::PluginList,
    logger: Box<dyn plugin::logging::LogWriter>,
}

impl Application {
    pub fn new(mut plugins: plugin::PluginList) -> Option<Self> {
        let logging = plugins.get_plugin::<plugin::Logging>();
        if logging.is_none() {
            println!("no logging");
            return None;
        }
        let logging = logging.unwrap();
        let ctx = logging.new_context("application".to_owned());
        let logger = Box::new(logging.new_logger(ctx));
        Some(Self {
            plugins,
            logger,
        })
    }
    pub fn run(&mut self) {
        //self.logger.log_raw("BEFORE run".to_owned());
        for plugin in &mut *self.plugins {
            plugin.run(&mut self.logger);
        }
        //self.logger.log_raw("AFTER run".to_owned());
    }
    pub fn shutdown(&mut self) {
        //self.logger.log_raw("BEFORE shutdown".to_owned());
        for plugin in self.plugins.iter_rev_mut() {
            plugin.shutdown(&mut self.logger);
        }
        //self.logger.log_raw("AFTER shutdown".to_owned());
    }
}

pub trait Command : AsCommand + Identifiable {
    fn execute(&mut self);
}

pub trait AsCommand {
    fn as_command(self) -> Rc<dyn Command>;
}

pub trait Identifiable {
    fn id(&self) -> Guid;
}

impl<T: Command + 'static> Identifiable for T {
    fn id(&self) -> Guid {
        Guid::new()
    }
}

impl<T: Command + 'static> AsCommand for T {
    fn as_command(self) -> Rc<dyn Command> {
        Rc::new(self)
    }
}

trait AggregateRoot {
}

// repository can only find and add aggregate roots
trait Repository<T: AggregateRoot> {
    fn find(&self, id: Guid) -> Rc<T>;
    fn add(&mut self, aggregate: T);
}

trait CommandHandler<T> {
    fn handle(&self, command: T);
}

struct Guid(uuid::Uuid);

impl Guid {
    pub fn new() -> Self {
        Self {
            0: uuid::Uuid::new_v4(),
        }
    }
}

impl fmt::Debug for Guid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.to_hyphenated())
    }
}
