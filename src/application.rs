use std::fmt;
use crate::plugin;
use std::rc::Rc;
use std::any::Any;

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

pub trait Command : AsCommand + AsAny {
}

pub trait AsCommand {
    fn as_command(self) -> Rc<dyn Command>;
}

pub trait AsAny {
    fn as_any(&self) -> &Any;
}

pub trait Identifiable {
    fn id(&self) -> &Guid;
}

impl<T: Command + 'static> AsCommand for T {
    fn as_command(self) -> Rc<dyn Command> {
        Rc::new(self)
    }
}

//impl<T: Command + 'static> AsAny for T {
//    fn as_any(&self) -> &Any {
//        self
//    }
//}

impl<T: Any> AsAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct IdentifiableCommand<T: Command> {
    guid: Guid,
    inner: T,
}

impl<T: Command + 'static> Command for IdentifiableCommand<T> {
}

impl<T: Command + 'static> Identifiable for IdentifiableCommand<T> {
    fn id(&self) -> &Guid {
        &self.guid
    }
}
//impl<T: Command + 'static> AsCommand for IdentifiableCommand<T> {
//    fn as_command(self) -> Rc<dyn Command> {
//        Rc::new(self.inner)
//    }
//}

pub trait AggregateRoot: AsAny {
}

// repository can only find and add aggregate roots
trait Repository<T: AggregateRoot> {
    fn find(&self, id: &Guid) -> Rc<T>;
    fn add(&mut self, aggregate: T);
}

pub trait CommandHandler {
    type Command: Command;
    fn handle(&self, command: Self::Command);
}

pub struct Guid(uuid::Uuid);

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

trait UnitOfWork {
    fn commit(&mut self);
    fn rollback(&mut self);
    //TODO: fn to track agg root, record as new agg root, record agg root deletion
}

struct AsyncUnitOfWork {
}

impl UnitOfWork for AsyncUnitOfWork {
    fn commit(&mut self) {
    }
    fn rollback(&mut self) {
    }
}

struct SyncUnitOfWork {
}

impl UnitOfWork for SyncUnitOfWork {
    fn commit(&mut self) {
    }
    fn rollback(&mut self) {
    }
}

trait Event {
    fn message_id(&self) -> &Guid;
    fn correlation_id(&self) -> &Guid;
    fn causation_id(&self) -> &Guid;
}

pub trait Handler<T: Command> {
    //TODO: fn handle(&mut self, command: &T, uow: dyn UnitOfWork) -> Vec<Rc<dyn Event>>;
    fn handle(&mut self, command: &T) -> Vec<Rc<dyn Event>>;
}

trait ApplyEvent<T: Event> {
    fn apply(&mut self, event: &T, historical: bool);
}

//saga: listens to events and issues commands
