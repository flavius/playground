use std::any::Any;

use crate::plugin;
use crate::logging;

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
        "appendlog"
    }

    fn id(&self) -> std::any::TypeId {
        std::any::TypeId::of::<Plugin>()
    }
    fn dependencies(&self) -> Vec<std::any::TypeId> {
        vec![
            std::any::TypeId::of::<logging::Plugin>(),
        ]
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
