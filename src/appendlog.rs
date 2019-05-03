use std::any::Any;

use crate::logging;
use crate::plugin;

pub struct Plugin {}

pub struct Specification {
    plugin: Option<Box<dyn plugin::Plugin>>,
}

impl Plugin {
    fn new(logging: logging::Plugin) -> Option<Self> {
        Some(Plugin {})
    }
}

impl plugin::Plugin for Plugin {}

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
}
