use crate::plugin;
use crate::logging;
use crate::projector;
use crate::appendlog;

pub struct Plugin {
}

pub struct Specification {
}

impl plugin::Specification for Specification {
    //type Plugin = Plugin;

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
}

impl plugin::Plugin for Plugin {
    type Specification = Specification;

    fn new(deps: Self::Specification) -> Self {
        Plugin {
        }
    }

    fn specification() -> Self::Specification {
        Specification {
        }
    }
}

