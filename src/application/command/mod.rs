mod me;
pub use me::Me;

use std::rc::Rc;

mod help;
pub use help::Help;

mod newtask;
pub use newtask::NewTask;

use crate::application;

pub struct CommandBus {
}

impl CommandBus {
    pub fn new() -> Self {
        Self {
        }
    }
    pub fn executeCommand(&mut self, mut command: Rc<dyn application::Command>) {
        //start unit of work (sync or async)
        //repositories registered with the UoW
        //trigger all handlers
        Rc::get_mut(&mut command).unwrap().execute();
        //when UoW is done, flush the repositories
    }
}
