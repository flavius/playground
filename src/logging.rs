use std::any::{Any, TypeId};
use std::collections::HashMap;

use crate::plugin;

pub struct Plugin {}

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
        Some(Plugin {})
    }
}

impl plugin::Plugin for Plugin {
    fn as_any(&self) -> &dyn Any {
        self
    }
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
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn new_plugin(&self, plugins: &Vec<Box<dyn plugin::Plugin>>) -> Result<Box<dyn plugin::Plugin>, plugin::PluginError> {
        match Plugin::new() {
            None => Err("cannot create logging plugin"),
            Some(plugin) => Ok(Box::new(plugin)),
        }
    }
}
