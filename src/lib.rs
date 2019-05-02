mod plugin;
use plugin::Plugin;
use plugin::Specification;
mod utils;
mod web;
mod logging;
mod appendlog;
mod projector;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initialization_order() {
        let spec_web = Box::new(web::Specification::new());
        let spec_logging = Box::new(logging::Specification::new());
        let spec_appendlog = Box::new(appendlog::Specification::new());
        let spec_projector = Box::new(projector::Specification::new());
        let deps : Vec<Box<dyn plugin::Specification>> = vec![
            spec_web,
            spec_logging,
            spec_appendlog,
            spec_projector,
        ];
        let sorted_specs = utils::sort_specifications(deps).unwrap();
        let expected = vec!["logging", "appendlog", "projector", "web"];
        let actual : Vec<&str> = sorted_specs.iter().map(|v| v.name()).collect();
        assert_eq!(expected, actual);
    }
}
