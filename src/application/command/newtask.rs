use crate::application::Command;
use crate::application::Guid;
use std::rc::Rc;
use crate::application::Identifiable;

extern crate proc_macro;

use proc_macro::{command};

#[command]
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
}

