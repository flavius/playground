use std::any::Any;

use crate::plugin;

pub struct Plugin {}

pub struct Specification {
    plugin: Option<Box<dyn plugin::Plugin>>,
}

impl Plugin {
    fn new() -> Option<Self> {
        Some(Plugin {})
    }
}
impl plugin::Plugin for Plugin {}

impl plugin::Specification for Specification {
    fn new() -> Self {
        Specification { plugin: None }
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
}
