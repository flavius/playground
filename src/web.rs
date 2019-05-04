use crate::appendlog;
use crate::logging;
use crate::plugin;
use crate::projector;

use std::any::{Any, TypeId};
use std::collections::HashMap;

pub struct Plugin {}

pub struct Specification {
    plugin: Option<Box<dyn plugin::Plugin>>,
}

impl plugin::Specification for Specification {
    fn new() -> Self {
        Specification { plugin: None }
    }

    fn name(&self) -> &'static str {
        "web"
    }

    fn id(&self) -> std::any::TypeId {
        std::any::TypeId::of::<Plugin>()
    }

    fn dependencies(&self) -> Vec<std::any::TypeId> {
        vec![
            std::any::TypeId::of::<logging::Plugin>(),
            std::any::TypeId::of::<appendlog::Plugin>(),
            std::any::TypeId::of::<projector::Plugin>(),
        ]
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn new_plugin(&self, plugins: &Vec<Box<dyn plugin::Plugin>>) -> Result<Box<dyn plugin::Plugin>, crate::InfrastructureError> {
        let log_plugin_idx = plugin::get_dep::<logging::Plugin>(plugins)?;
        let log_plugin = plugins[log_plugin_idx].as_any().downcast_ref::<logging::Plugin>().unwrap();
        let appendlog_plugin_idx = plugin::get_dep::<appendlog::Plugin>(plugins)?;
        let appendlog_plugin = plugins[appendlog_plugin_idx].as_any().downcast_ref::<appendlog::Plugin>().unwrap();
        let projector_plugin_idx = plugin::get_dep::<projector::Plugin>(plugins)?;
        let projector_plugin = plugins[projector_plugin_idx].as_any().downcast_ref::<projector::Plugin>().unwrap();
        match Plugin::new(log_plugin, appendlog_plugin, projector_plugin) {
            None => Err("cannot create web plugin"),
            Some(plugin) => Ok(Box::new(plugin)),
        }
    }
}

impl plugin::Plugin for Plugin {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Plugin {
    fn new(_logging: &logging::Plugin, _appendlog: &appendlog::Plugin, projector: &projector::Plugin) -> Option<Self> {
        Some(Plugin {})
    }
}
