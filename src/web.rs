use crate::plugin;
use crate::logging;
use crate::projector;
use crate::appendlog;

use std::any::Any;

pub struct Plugin {
}

pub struct Specification {
}

impl plugin::Specification for Specification {
    fn new() -> Self {
        Specification {
        }
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

    fn new(deps: Box<dyn plugin::Specification>) -> Option<Self> {
        Some(Plugin {
        })
    }
}

