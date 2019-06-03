use crate::application::{Handler, command, Event, AggregateRoot};
use std::rc::Rc;

pub struct Tasklist {
}

impl Tasklist {
    pub fn new() -> Self {
        Self {
        }
    }
    fn add_todo(&mut self, task: String) {
    }
}

impl Handler<command::NewTask> for Tasklist {
    fn handle(&mut self, command: &command::NewTask) -> Vec<Rc<dyn Event>> {
        println!("handled newtask by domain!");
        self.add_todo(command.as_string());
        vec![]
    }
}

impl AggregateRoot for Tasklist {
}
