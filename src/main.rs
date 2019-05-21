use std::rc::Rc;

enum Plugin {
    Web(Web),
    Logging(Logging),
}

impl Plugin {
    fn run(&self) {
        println!("running generic plugin");
        match *self {
            Plugin::Web(ref plugin) => plugin.run(),
            Plugin::Logging(ref plugin) => plugin.run(),
        }
        println!("finish running generic plugin");
    }
    fn shutdown(&self) {
        println!("shutdown generic plugin");
        match *self {
            Plugin::Web(ref plugin) => plugin.shutdown(),
            Plugin::Logging(ref plugin) => plugin.shutdown(),
        }
        println!("finish shutdown generic plugin");
    }
}

struct Web {
}

impl Web {
    fn run(&self) {
        println!("running web plugin");
    }
    fn shutdown(&self) {
        println!("shutdown web plugin");
    }
}

struct Logging {
}

impl Logging {
    fn new() -> Self {
        Logging {
        }
    }
    fn run(&self) {
        println!("running logging plugin");
    }
    fn shutdown(&self) {
        println!("shutdown logging plugin");
    }
}

struct PluginList(Vec<Rc<Plugin>>);

impl PluginList {
    fn new() -> Self {
        PluginList (
            vec![],
        )
    }
    fn all(&self) -> impl Iterator<Item = &Rc<Plugin>> {
        self.0.iter()
    }
    fn all_rev(&self) -> impl DoubleEndedIterator<Item = &Rc<Plugin>> {
        self.0.iter().rev()
    }
    fn register(&mut self, plugin: Plugin) {
        self.0.push(Rc::new(plugin));
    }
}

struct Application {
    plugins: PluginList,
}

impl Application {
    fn new() -> Self {
        let mut plugins = PluginList::new();
        let logging = Logging::new();
        plugins.register(Plugin::Logging(logging));
        Application {
            plugins,
        }
    }
    fn run(&self) {
        println!("running app");
        for plugin in self.plugins.all() {
            plugin.run();
        }
        println!("finished running app");
    }
    fn shutdown(&self) {
        println!("shutting down app");
        for plugin in self.plugins.all_rev() {
            plugin.shutdown();
        }
        println!("finished shutting down app");
    }
}
fn main() {
    let app = Application::new();
    app.run();
    app.shutdown();
}
