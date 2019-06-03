//use crate::application::{Command, AsCommand, Handler};
use crate::application::{Command};

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

    pub fn as_string(&self) -> String {
        self.rawtaskdesc.join(" ")
    }
}

