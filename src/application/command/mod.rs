use std::rc::{Rc, Weak};
use std::collections::HashMap;
use std::any::TypeId;

mod me;
pub use me::Me;

mod help;
pub use help::Help;

mod newtask;
pub use newtask::NewTask;

mod tasklist;
pub use tasklist::Tasklist;

use crate::application::{Handler, Command};

pub struct CommandBus {
    handlers: HashMap<TypeId, Vec<Weak<Handler<Command>>>>,
}

impl CommandBus {
    pub fn new() -> Self {
        let mut handlers = HashMap::new();
        //let mut newtask_handlers = vec![];
        //newtask_handlers.push(Rc::new(tasklist as Handler<Command>));
        //handlers.insert(
        //    TypeId::of::<NewTask>(),
        //    newtask_handlers,
        //);
        Self {
            handlers,
        }
    }
    //pub fn aggregateroot_handles<C: Command + 'static>(&mut self, aggroot: &Rc<dyn AggregateRoot>) -> bool {
    pub fn aggregateroot_handles<C: Command + 'static>(&mut self, aggroot: &Rc<Handler<C>>) -> bool {
        let command_id = TypeId::of::<C>();
        let typehandlers = match self.handlers.get_mut(&command_id) {
            Some(v) => v,
            None => {
                let v = vec![];
                self.handlers.insert(command_id, v);
                self.handlers.get_mut(&command_id).unwrap()
            }
        };
        //let weak_aggroot: Weak<Handler<Command>> = Rc::downgrade(aggroot as &Rc<Handler<Command>>);
        false
        //let any_aggroot = Rc::downcast::<Handler<Command>>(aggroot.as_any());
        //match any_aggroot {
        //    Ok(v) => {
        //        typehandlers.push(Rc::downgrade(&v));
        //        true
        //    },
        //    Err(_) => {
        //        false
        //    }
        //}
    }
    pub fn executeCommand(&mut self, mut command: Rc<dyn Command>) {
        println!("BUS EXECUTING");
        let mut command = Rc::get_mut(&mut command).unwrap();
        println!("cmd: {:?}", &command.as_any().type_id());
        println!("END BUS EXECUTING");
        //start unit of work (sync or async)
        //repositories registered with the UoW
        //trigger all handlers
        //when UoW is done, flush the repositories
    }
    pub fn execute_command<T: Command>(&mut self, mut command: T) {
    }
}
