use std::rc::{Rc, Weak};
use std::collections::HashMap;
use std::any::{TypeId, Any};

mod me;
pub use me::Me;

mod help;
pub use help::Help;

mod newtask;
pub use newtask::NewTask;

mod tasklist;
pub use tasklist::Tasklist;

use crate::application::{Handler, Command, AggregateRoot, AsAny};

struct HandlerList<C: Command> {
    handlers: Vec<Weak<dyn Handler<C>>>,
}

pub struct CommandBus {
    handlers_map: HashMap<TypeId, Box<dyn Any>>,
}

impl CommandBus {
    pub fn new() -> Self {
        let handlers_map = HashMap::new();
        Self {
            handlers_map,
        }
    }
    pub fn aggregateroot_handles<C, H>(&mut self, aggroot: &Rc<H>) -> bool
        where C: Command + 'static,
              H: Handler<C> + 'static {
        let id = TypeId::of::<C>();
        let handler_list: &mut HandlerList<C> = self
            .handlers_map
            .entry(id)
            .or_insert_with(|| Box::new(HandlerList::<C> { handlers: Vec::new() }))
            .downcast_mut()
            .expect("broke typeid invariant")
            ;
        let handler = Rc::downgrade(aggroot) as Weak<dyn Handler<C>>;
        //let down_id = &handler.type_id();
        //println!("command {:?} handled by {:?} down_id {:?}", &id, &aggroot.as_any().type_id(), &down_id);
        handler_list.handlers.push(handler);
        true
    }
    pub fn executeCommand<C: Command + 'static>(&mut self, command: Rc<C>) {
        println!("BUS EXECUTING");
        let id = TypeId::of::<C>();
        if let Some(handlers) = self.handlers_map.get(&id) {
            let handler_list: &HandlerList<C> = handlers.downcast_ref().expect("could not downcast handlers");
            for handler in &handler_list.handlers {
                let handler = handler.upgrade().unwrap();
                handler.handle(&command);
            }
        }
        println!("END BUS EXECUTING");
        //start unit of work (sync or async)
        //repositories registered with the UoW
        //trigger all handlers
        //when UoW is done, flush the repositories
    }
    pub fn execute_command<T: Command>(&mut self, mut command: T) {
    }
}
