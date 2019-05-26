extern crate rust_ddd;

use std::env;

use rust_ddd::{plugin, Application, PluginList, MyPlugin};

fn main() {
    let logging = plugin::Logging::new();
    let appendlog = plugin::Appendlog::new(&logging);
    let projector = plugin::Projector::new(&appendlog, &logging);
    let mut args : Vec<String> = env::args().collect();
    let called_as = args.remove(0);
    let cli = plugin::Cli::new(args, env::vars().collect(), &logging);

    let mut plugins = PluginList::new();

    plugins.register(MyPlugin::Logging(logging));
    plugins.register(MyPlugin::Projector(projector));
    plugins.register(MyPlugin::Appendlog(appendlog));
    plugins.register(MyPlugin::Cli(cli));

    let app = Application::new(plugins);
    if app.is_none() {
        return;
    }
    let mut app = app.unwrap();
    app.run();
    app.shutdown();
}

