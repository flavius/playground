use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::rc::Rc;

use crate::logging;
use crate::plugin;

pub struct Plugin<'a> {
    logger: &'a Rc<logging::Plugin>,
}

pub struct Specification {
}

impl<'a> Plugin<'a> {
    fn new(logger: &'a Rc<logging::Plugin>) -> Option<Self> {
        Some(Plugin {
            logger
        })
    }
}

impl<'a> plugin::Plugin for Plugin<'a> {
    fn as_any(&self) -> &dyn Any {
        self
    }
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
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn new_plugin(&self, plugins: &Vec<Rc<dyn plugin::Plugin>>) -> Result<Rc<dyn plugin::Plugin>, plugin::PluginError> {
        let log_plugin_idx = plugin::get_dep::<logging::Plugin>(plugins)?;
        let log_plugin = plugins[log_plugin_idx].as_any().downcast_ref::<Rc<logging::Plugin>>().unwrap();
        println!("XXX LOG PLUGIN: {:?}", log_plugin);
        match Plugin::new(log_plugin) {
            None => Err("cannot create appendlog plugin"),
            Some(plugin) => Ok(Rc::new(plugin)),
        }
    }
}
