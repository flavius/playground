use crate::appendlog;
use crate::logging;
use crate::plugin;
use crate::projector;

use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Plugin {}

impl Plugin {
    fn new(logger: Rc<RefCell<logging::Plugin>>, appendlog: Rc<RefCell<appendlog::Plugin>>, projector: Rc<RefCell<projector::Plugin>>) -> Option<Self> {
        Some(Plugin {})
    }
}

pub struct Specification {
}

impl plugin::Specification for Specification {
    fn new() -> Self {
        Specification {}
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

    fn new_plugin(&self, plugins: &Vec<Rc<RefCell<dyn plugin::Plugin>>>) -> Result<Rc<RefCell<dyn plugin::Plugin>>, plugin::PluginError> {
        let log_plugin = plugin::get_dep::<logging::Plugin>(plugins)?;
        let appendlog_plugin = plugin::get_dep::<appendlog::Plugin>(plugins)?;
        let projector_plugin = plugin::get_dep::<projector::Plugin>(plugins)?;
        match Plugin::new(log_plugin, appendlog_plugin, projector_plugin) {
            None => Err("cannot create web plugin"),
            Some(plugin) => Ok(Rc::new(RefCell::new(plugin))),
        }
    }
}

impl plugin::Plugin for Plugin {
    fn run(&mut self) {
        println!("running web");
    }
    fn shutdown(&self) {
        println!("shutting down web");
    }
}


