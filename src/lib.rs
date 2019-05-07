mod plugin;
mod appendlog;
mod logging;
mod utils;


use plugin::Plugin;
use plugin::Specification;

use std::any::{Any, TypeId};
use std::rc::Rc;

#[cfg(test)]
mod tests {
    use super::*;

    fn get_std_deps() -> Vec<Rc<dyn plugin::Specification>> {
        let spec_logging = Rc::new(logging::Specification::new());
        let spec_appendlog = Rc::new(appendlog::Specification::new());
        vec![spec_logging, spec_appendlog]
    }

    #[test]
    fn initialization_order() {
        let deps = get_std_deps();
        let sorted_specs = utils::sort_specifications(deps).unwrap();
        let expected = vec!["logging", "appendlog", ];
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
