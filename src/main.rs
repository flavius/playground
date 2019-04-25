mod utils;
mod web;
mod logging;
mod appendlog;
mod projector;
mod plugin;

use crate::plugin::Plugin;

fn main() {
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
    let sorted_specs = utils::sort_specifications(deps);
}
