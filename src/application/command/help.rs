use crate::application::{Command, AsCommand};

extern crate proc_macro;
use proc_macro::{command};

#[command]
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

