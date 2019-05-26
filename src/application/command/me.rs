use crate::application::Command;
use std::collections::HashMap;
use std::rc::Rc;

pub struct Me {
    contact: String,
    nickname: String,
}

impl Me {
    pub fn new(contact: String, nickname: String) -> Self {
        Self {
            contact,
            nickname,
        }
    }
    pub fn as_command(self) -> Rc<dyn Command> {
        Rc::new(self)
    }
}

impl Command for Me {
    fn execute(&mut self) {
        println!("executing ME");
    }
    fn id(&self) {
    }
}
