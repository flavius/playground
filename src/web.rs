use crate::appendlog;
use crate::logging;
use crate::plugin;
use crate::projector;

use std::any::Any;

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
}

impl plugin::Plugin for Plugin {
    //type Specification = Specification;
}

impl Plugin {
    fn new(
        logging: logging::Plugin,
        appendlog: appendlog::Plugin,
        projector: projector::Plugin,
    ) -> Option<Self> {
        Some(Plugin {})
    }
}
