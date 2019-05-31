use crate::application::{Command, AsCommand};

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

