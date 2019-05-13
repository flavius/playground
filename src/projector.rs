use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

use crate::appendlog;
use crate::logging;
use crate::plugin;

pub struct Plugin {}

pub struct Specification {
}

impl Plugin {
    fn new(logger: Rc<RefCell<logging::Plugin>>, appendlog: Rc<RefCell<appendlog::Plugin>>) -> Option<Self> {
        Some(Plugin {})
    }
}

impl plugin::Plugin for Plugin {
    fn run(&mut self) {
        println!("running projector");
    }
    fn shutdown(&self) {
        println!("shutting down projector");
    }
}

impl plugin::Specification for Specification {
    fn new() -> Self {
        Specification {}
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
    fn new_plugin(&self, plugins: &Vec<Rc<RefCell<dyn plugin::Plugin>>>) -> Result<Rc<RefCell<dyn plugin::Plugin>>, plugin::PluginError> {
        let log_plugin = plugin::get_dep::<logging::Plugin>(plugins)?;
        let appendlog_plugin = plugin::get_dep::<appendlog::Plugin>(plugins)?;
        match Plugin::new(log_plugin, appendlog_plugin) {
            None => Err("cannot create projector plugin"),
            Some(plugin) => Ok(Rc::new(RefCell::new(plugin))),
        }
    }
}

