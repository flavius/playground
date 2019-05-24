extern crate rust_ddd;

use rust_ddd::{plugin, Application, PluginList, MyPlugin};

fn main() {
    let logging = plugin::Logging::new();
    let cli = plugin::Cli::new(&logging);
    let mut plugins = PluginList::new();
    plugins.register(MyPlugin::Logging(logging));
    plugins.register(MyPlugin::Cli(cli));
    let app = Application::new(plugins);
    if app.is_none() {
        return;
    }
    let mut app = app.unwrap();
    app.run();
    app.shutdown();
}

