fn main() {
    let logging = Logging::new();
    let web = Web::new(&logging);
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
