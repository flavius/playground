use crate::plugin;

pub struct Plugin {
}

pub struct Specification {
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

impl plugin::Specification for Specification {
    fn name(&self) -> &'static str {
        "logging"
    }

    fn id(&self) -> std::any::TypeId {
        std::any::TypeId::of::<Plugin>()
    }
}

