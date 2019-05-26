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
}

impl Command for Me {
    fn execute(&mut self) {
        println!("executing ME for {} known as {}", &self.contact, &self.nickname);
    }
    fn id(&self) {
    }
}
