use crate::application::Command;
use std::collections::HashMap;
use std::rc::Rc;

pub struct Help {
}

impl Help {
    pub fn new() -> Self {
        Self {
        }
    }
    pub fn as_command(self) -> Rc<dyn Command> {
        Rc::new(self)
    }
}

impl Command for Help {
    fn execute(&mut self) {
        println!("executing help");
    }
    fn id(&self) {
    }
}

