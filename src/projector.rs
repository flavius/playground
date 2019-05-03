use std::any::Any;

use crate::appendlog;
use crate::logging;
use crate::plugin;

pub struct Plugin {}

pub struct Specification {
    plugin: Option<Box<dyn plugin::Plugin>>,
}

impl Plugin {
    fn new(_logging: logging::Plugin, _appendlog: appendlog::Plugin) -> Option<Self> {
        Some(Plugin {})
    }
}
impl plugin::Plugin for Plugin {}

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
}
