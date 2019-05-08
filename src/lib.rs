mod plugin;
mod appendlog;
mod logging;
mod projector;
mod utils;
mod web;


use plugin::Plugin;
use plugin::Specification;

use std::any::{Any, TypeId};
use std::rc::Rc;

#[cfg(test)]
mod tests {
    use super::*;

    fn get_std_deps() -> Vec<Rc<dyn plugin::Specification>> {
        let spec_web = Rc::new(web::Specification::new());
        let spec_logging = Rc::new(logging::Specification::new());
        let spec_appendlog = Rc::new(appendlog::Specification::new());
        let spec_projector = Rc::new(projector::Specification::new());
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
        for s in sorted_specs.iter() {
            println!("{} {:?}", s.name(), s.id());
        }
        let expected_ids: Vec<TypeId> = sorted_specs.iter().map(|v| v.id()).collect();

        let plugins = utils::initialize_plugins(sorted_specs).unwrap();

        let actual_ids: Vec<TypeId> = plugins.iter().map(|v| v.as_any().type_id()).collect();
        assert_eq!(expected_ids, actual_ids);
    }

    #[test]
    fn plugin_lifecycle() {
        let deps = get_std_deps();
        let sorted_specs = utils::sort_specifications(deps).unwrap();
        let plugins = utils::initialize_plugins(sorted_specs).unwrap();
        for plugin in plugins.iter() {
            plugin.run();
        }
        for plugin in plugins.iter().rev() {
            plugin.shutdown();
        }
    }
}
