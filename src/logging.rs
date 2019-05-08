use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::rc::Rc;

use crate::plugin;

pub struct Plugin {
    count: usize,
}

impl ::std::fmt::Debug for Plugin {
    fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let mut builder = __arg_0.debug_struct("logging::Plugin");
        builder.finish()
    }
}

pub struct Specification {
}

impl Plugin {
    fn new() -> Option<Self> {
        Some(Plugin {
            count: 0,
        })
    }
    fn logSomething(&mut self) {
        self.count += 1;
    }
}

impl plugin::Plugin for Plugin {
    fn run(&self) {
        println!("running logging");
    }
    fn shutdown(&self) {
        println!("shutting down logging");
    }
}

impl plugin::Specification for Specification {
    fn new() -> Self {
        Specification { }
    }
    fn name(&self) -> &'static str {
        "logging"
    }

    fn id(&self) -> std::any::TypeId {
        std::any::TypeId::of::<Plugin>()
    }
    fn new_plugin(&self, plugins: &Vec<Rc<dyn plugin::Plugin>>) -> Result<Rc<dyn plugin::Plugin>, plugin::PluginError> {
        match Plugin::new() {
            None => Err("cannot create logging plugin"),
            Some(plugin) => Ok(Rc::new(plugin)),
        }
    }
}

