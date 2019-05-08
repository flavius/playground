use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::rc::Rc;

use crate::logging;
use crate::plugin;

#[derive(Debug)]
pub struct Plugin {
    logger: Rc<logging::Plugin>,
}

pub struct Specification {
}

impl Plugin {
    fn new(logger: Rc<logging::Plugin>) -> Option<Self> {
        Some(Plugin {
            logger,
        })
    }
}

impl plugin::Plugin for Plugin {
    fn run(&self) {
        println!("running appendlog");
    }
    fn shutdown(&self) {
        println!("shutting down appendlog");
    }
}

impl plugin::Specification for Specification {
    fn new() -> Self {
        Specification {}
    }
    fn name(&self) -> &'static str {
        "appendlog"
    }

    fn id(&self) -> std::any::TypeId {
        std::any::TypeId::of::<Plugin>()
    }
    fn dependencies(&self) -> Vec<std::any::TypeId> {
        vec![std::any::TypeId::of::<logging::Plugin>()]
    }
    fn new_plugin(&self, plugins: &Vec<Rc<dyn plugin::Plugin>>) -> Result<Rc<dyn plugin::Plugin>, plugin::PluginError> {
        let log_plugin = plugin::get_dep::<logging::Plugin>(plugins)?;
        match Plugin::new(log_plugin) {
            None => Err("cannot create appendlog plugin"),
            Some(plugin) => Ok(Rc::new(plugin)),
        }
    }
}

