use std::ops::{Deref, DerefMut};
use std::any::{Any, TypeId};

enum MyPlugin {
    Web(Web),
    Logging(Logging),
}

trait Plugin : Any + AsAnyPlugin {
    fn run(&mut self);
    fn shutdown(&mut self);
}

trait AsAnyPlugin {
    fn as_any(self: &Self) -> &dyn Any;
    fn as_any_mut(self: &mut Self) -> &mut dyn Any;
}

impl<T: Plugin + 'static> AsAnyPlugin for T {
    fn as_any(self: &Self) -> &dyn Any {
        self
    }
    fn as_any_mut(self: &mut Self) -> &mut dyn Any {
        self
    }
}

impl MyPlugin {
    fn run(&mut self, logger: &mut Box<dyn LogWriter>) {
        logger.log_raw("BEFORE casted run".to_owned());
        self.as_plugin_mut().run();
        logger.log_raw("AFTER casted run".to_owned());
    }
    fn shutdown(&mut self, logger: &mut Box<dyn LogWriter>) {
        logger.log_raw("BEFORE casted shutdown".to_owned());
        self.as_plugin_mut().shutdown();
        logger.log_raw("AFTER casted shutdown".to_owned());
    }

    // match boilerplate is centralized here
    // Avoidable and optimizable with crates such as "enum_dispatch"
    fn as_plugin_mut(&mut self) -> &mut dyn Plugin {
        match self {
            MyPlugin::Web(plugin) => plugin,
            MyPlugin::Logging(plugin) => plugin,
        }
    }
    fn as_plugin(&mut self) -> &dyn Plugin {
        self.as_plugin_mut()
    }
    fn plugin_id(&mut self) -> TypeId {
        let plugin = self.as_plugin_mut();
        let any_plugin = plugin.as_any();
        any_plugin.type_id()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

struct Web {
    logger: Box<dyn LogWriter>,
}

impl Web {
    fn new(logging: &Logging) -> Self {
        let ctx = logging.new_context("web".to_owned());
        let logger = Box::new(logging.new_logger(ctx));
        Self {
            logger,
        }
    }
}

impl Plugin for Web {
    fn run(&mut self) {
        self.logger.log_raw("run".to_owned());
    }
    fn shutdown(&mut self) {
        self.logger.log_raw("shutdown".to_owned());
    }
}

struct Logging {
}

impl Logging {
    fn new() -> Self {
        Self {
        }
    }

    fn new_logger(&self, context: LoggingContext) -> impl LogWriter {
        InMemoryLogger::new(context)
    }
    fn new_context(&self, id: String) -> LoggingContext {
        LoggingContext(id)
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Plugin for Logging {
    fn run(&mut self) {
        let ctx = self.new_context("logging".to_owned());
        let mut logger = Box::new(self.new_logger(ctx));
        logger.log_raw("run".to_owned());
    }
    fn shutdown(&mut self) {
        let ctx = self.new_context("logging".to_owned());
        let mut logger = Box::new(self.new_logger(ctx));
        logger.log_raw("shutdown".to_owned());
    }
}

trait LogWriter {
    fn log_raw(&mut self, msg: String);
}

struct InMemoryLogger {
    messages: Vec<String>,
    context: LoggingContext,
}

impl LogWriter for InMemoryLogger {
    fn log_raw(&mut self, msg: String) {
        println!("{}\t\t{}", &self.context.0, &msg);
        self.messages.push(msg);
    }
}

impl InMemoryLogger {
    fn new(context: LoggingContext) -> Self {
        let messages = vec![];
        Self {
            messages,
            context,
        }
    }
}

struct PluginList(Vec<MyPlugin>);

struct LoggingContext(String);

impl PluginList {
    fn new() -> Self {
        Self (
            vec![],
        )
    }
    fn register(&mut self, mut my_plugin: MyPlugin) {
        let plugin_id = my_plugin.plugin_id();
        self.0.push(my_plugin);
    }
    fn get_plugin<'a, T: Plugin + 'static + 'a>(&'a mut self) -> Option<&'a mut T> {
        let type_id = TypeId::of::<T>();
        for my_plugin in self.0.iter_mut() {
            if my_plugin.plugin_id() == type_id {
                let mut plugin = my_plugin.as_plugin_mut();
                return plugin.as_any_mut().downcast_mut::<T>();
            }
        }
        None
    }
    fn iter_rev_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut MyPlugin> {
        self.0.iter_mut().rev()
    }
}

impl Deref for PluginList {
    type Target = Vec<MyPlugin>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for PluginList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

struct Application {
    plugins: PluginList,
    logger: Box<dyn LogWriter>,
}

impl Application {
    fn new(mut plugins: PluginList) -> Option<Self> {
        let logging: Option<&mut Logging> = plugins.get_plugin::<Logging>();
        if logging.is_none() {
            println!("no logging");
            return None;
        }
        let logging = logging.unwrap();
        let ctx = logging.new_context("application".to_owned());
        let logger = Box::new(logging.new_logger(ctx));
        Some(Self {
            plugins,
            logger,
        })
    }
    fn run(&mut self) {
        self.logger.log_raw("BEFORE run".to_owned());
        for plugin in &mut *self.plugins {
            plugin.run(&mut self.logger);
        }
        self.logger.log_raw("AFTER run".to_owned());
    }
    fn shutdown(&mut self) {
        self.logger.log_raw("BEFORE shutdown".to_owned());
        for plugin in self.plugins.iter_rev_mut() {
            plugin.shutdown(&mut self.logger);
        }
        self.logger.log_raw("AFTER shutdown".to_owned());
    }
}
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
