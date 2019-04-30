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
        let spec_web = web::Plugin::specification();
        let spec_logging = logging::Plugin::specification();
        let spec_appendlog = appendlog::Plugin::specification();
        let spec_projector = projector::Plugin::specification();
        let deps : Vec<&plugin::Specification> = vec![
            &spec_web,
            &spec_logging,
            &spec_appendlog,
            &spec_projector,
        ];
        let sorted_specs = utils::sort_specifications(deps).unwrap();
        let expected = vec!["logging", "appendlog", "projector", "web"];
        let actual = sorted_specs.map(|v| v.name());
        assert_eq!(expected, actual);
    }
}
