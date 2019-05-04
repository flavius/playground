mod plugin;
use plugin::Plugin;
use plugin::Specification;
mod appendlog;
mod logging;
mod projector;
mod utils;
mod web;

type InfrastructureError = &'static str;

use std::any::{Any, TypeId};

#[cfg(test)]
mod tests {
    use super::*;

    fn get_std_deps() -> Vec<Box<dyn plugin::Specification>> {
        let spec_web = Box::new(web::Specification::new());
        let spec_logging = Box::new(logging::Specification::new());
        let spec_appendlog = Box::new(appendlog::Specification::new());
        let spec_projector = Box::new(projector::Specification::new());
        vec![spec_web, spec_logging, spec_appendlog, spec_projector]
    }

    #[test]
    fn initialization_order() {
        let deps = get_std_deps();
        let sorted_specs = utils::sort_specifications(deps).unwrap();
        let expected = vec!["logging", "appendlog", "projector", "web"];
        let actual: Vec<&str> = sorted_specs.iter().map(|v| v.name()).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn instantiate_plugins() {
        let deps = get_std_deps();
        let sorted_specs = utils::sort_specifications(deps).unwrap();
        let expected_ids: Vec<TypeId> = sorted_specs.iter().map(|v| v.id()).collect();

        let plugins = utils::initialize_plugins(sorted_specs).unwrap();

        let actual_ids: Vec<TypeId> = plugins.iter().map(|v| v.as_any().type_id()).collect();
        assert_eq!(expected_ids, actual_ids);
    }
}
