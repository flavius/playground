use crate::application::Command;
use std::collections::HashMap;
use std::rc::Rc;

pub struct Help {
    indirect_call: bool,
    original_args: Vec<String>,
}

impl Help {
    pub fn new(indirect_call: bool, original_args: Vec<String>) -> Self {
        Self {
            indirect_call,
            original_args,
        }
    }
}

impl Command for Help {
    fn execute(&mut self) {
        if self.indirect_call {
            println!("helping for original args: {:?}", &self.original_args);
        } else {
            println!("regular help");
        }
    }
}

