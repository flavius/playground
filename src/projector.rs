use std::any::{Any, TypeId};
use std::collections::HashMap;

use crate::appendlog;
use crate::logging;
use crate::plugin;

pub struct Plugin {}

pub struct Specification {
    plugin: Option<Box<dyn plugin::Plugin>>,
}

impl Plugin {
    fn new(_logging: &logging::Plugin, _appendlog: &appendlog::Plugin) -> Option<Self> {
        Some(Plugin {})
    }
}

impl plugin::Plugin for Plugin {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn run(&self) {
        println!("running projector");
    }
    fn shutdown(&self) {
        println!("shutting down projector");
    }
}

impl plugin::Specification for Specification {
    fn new() -> Self {
        Specification { plugin: None }
    }
    fn name(&self) -> &'static str {
        "projector"
    }
    fn id(&self) -> std::any::TypeId {
        std::any::TypeId::of::<Plugin>()
    }
    fn dependencies(&self) -> Vec<std::any::TypeId> {
        vec![
            std::any::TypeId::of::<logging::Plugin>(),
            std::any::TypeId::of::<appendlog::Plugin>(),
        ]
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn new_plugin(&self, plugins: &Vec<Box<dyn plugin::Plugin>>) -> Result<Box<dyn plugin::Plugin>, plugin::PluginError> {
        let log_plugin_idx = plugin::get_dep::<logging::Plugin>(plugins)?;
        let log_plugin = plugins[log_plugin_idx].as_any().downcast_ref::<logging::Plugin>().unwrap();
        let appendlog_plugin_idx = plugin::get_dep::<appendlog::Plugin>(plugins)?;
        let appendlog_plugin = plugins[appendlog_plugin_idx].as_any().downcast_ref::<appendlog::Plugin>().unwrap();
        match Plugin::new(log_plugin, appendlog_plugin) {
            None => Err("cannot create projector plugin"),
            Some(plugin) => Ok(Box::new(plugin)),
        }
    }
}
