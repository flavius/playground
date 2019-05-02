use std::any::Any;

use crate::plugin;
use crate::logging;
use crate::appendlog;

pub struct Plugin {
}

pub struct Specification {
}

impl plugin::Plugin for Plugin {
    fn new(deps: Box<dyn plugin::Specification>) -> Option<Self> {
        Some(Plugin {
        })
    }

}

impl plugin::Specification for Specification {
    fn new() -> Self {
        Specification {
        }
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

