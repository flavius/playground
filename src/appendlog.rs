use std::any::{Any, TypeId};
use std::collections::HashMap;

use crate::logging;
use crate::plugin;

#[derive(Debug)]
pub struct Plugin {}

pub struct Specification {
    plugin: Option<Box<dyn plugin::Plugin>>,
}

impl Plugin {
    fn new(_logging: &logging::Plugin) -> Option<Self> {
        Some(Plugin {})
    }
}

impl plugin::Plugin for Plugin {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl plugin::Specification for Specification {
    fn new() -> Self {
        Specification { plugin: None }
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
    fn new_plugin(&self, plugins: &Vec<Box<dyn plugin::Plugin>>) -> Result<Box<dyn plugin::Plugin>, crate::InfrastructureError> {
        let log_plugin_idx = plugin::get_dep::<logging::Plugin>(plugins)?;
        let log_plugin = plugins[log_plugin_idx].as_any().downcast_ref::<logging::Plugin>().unwrap();
        match Plugin::new(log_plugin) {
            None => Err("cannot create appendlog plugin"),
            Some(plugin) => Ok(Box::new(plugin)),
        }
    }
}
