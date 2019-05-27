use crate::application::Command;
use std::rc::Rc;
use crate::application::Identifiable;

pub struct NewTask {
    rawtaskdesc: Vec<String>,
}

impl NewTask {
    pub fn new(rawtaskdesc: Vec<String>) -> Self {
        Self {
            rawtaskdesc,
        }
    }
}

impl Command for NewTask {
    fn execute(&mut self) {
        println!("creating new task by command {:?}", &self.id());
    }
}

