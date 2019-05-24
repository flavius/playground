extern crate rust_ddd;

use rust_ddd::{plugin, Application, PluginList, MyPlugin};

fn main() {
    let logging = plugin::Logging::new();
    let web = plugin::Web::new(&logging);
    let mut plugins = PluginList::new();
    plugins.register(MyPlugin::Logging(logging));
    plugins.register(MyPlugin::Web(web));
    let app = Application::new(plugins);
    if app.is_none() {
        return;
    }
    let mut app = app.unwrap();
    app.run();
    app.shutdown();
}
