use std::rc::Rc;

enum Plugin {
    Web(Web),
    Logging(Logging),
}

impl Plugin {
    fn run(&mut self, logger: &mut Box<dyn LogWriter>) {
        logger.log_raw("BEFORE casted run".to_owned());
        match *self {
            Plugin::Web(ref mut plugin) => plugin.run(),
            Plugin::Logging(ref mut plugin) => plugin.run(),
        }
        logger.log_raw("AFTER casted run".to_owned());
    }
    fn shutdown(&mut self, logger: &mut Box<dyn LogWriter>) {
        logger.log_raw("BEFORE casted shutdown".to_owned());
        match *self {
            Plugin::Web(ref mut plugin) => plugin.shutdown(),
            Plugin::Logging(ref mut plugin) => plugin.shutdown(),
        }
        logger.log_raw("AFTER casted shutdown".to_owned());
    }

    fn as_logging(&self) -> Option<&Logging> {
        match *self {
            Plugin::Logging(ref plugin) => Some(&plugin),
            _ => None,
        }
    }
}

struct Web {
    logger: Box<dyn LogWriter>,
}

impl Web {
    fn new(logging: &Logging) -> Self {
        let ctx = logging.new_context("web".to_owned());
        let logger = Box::new(logging.new_logger(ctx));
        Web {
            logger,
        }
    }
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
        Logging {
        }
    }
    fn run(&self) {
        let ctx = self.new_context("logging".to_owned());
        let mut logger = Box::new(self.new_logger(ctx));
        logger.log_raw("run".to_owned());
    }
    fn shutdown(&self) {
        let ctx = self.new_context("logging".to_owned());
        let mut logger = Box::new(self.new_logger(ctx));
        logger.log_raw("shutdown".to_owned());
    }

    fn new_logger(&self, context: LoggingContext) -> impl LogWriter {
        InMemoryLogger::new(context)
    }
    fn new_context(&self, id: String) -> LoggingContext {
        LoggingContext(id)
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
        InMemoryLogger {
            messages,
            context,
        }
    }
}

struct PluginList(Vec<Rc<Plugin>>);

struct LoggingContext(String);

impl PluginList {
    fn new() -> Self {
        PluginList (
            vec![],
        )
    }
    fn all(&self) -> impl Iterator<Item = &Rc<Plugin>> {
        self.0.iter()
    }
    fn all_mut(&mut self) -> impl Iterator<Item = &mut Rc<Plugin>> {
        self.0.iter_mut()
    }
    fn all_rev(&self) -> impl DoubleEndedIterator<Item = &Rc<Plugin>> {
        self.0.iter().rev()
    }
    fn all_rev_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Rc<Plugin>> {
        self.0.iter_mut().rev()
    }
    fn register(&mut self, plugin: Plugin) {
        self.0.push(Rc::new(plugin));
    }
    fn logging(&self) -> Rc<&Logging> {
        for plugin in self.all() {
            if let Some(v) = plugin.as_logging() {
                return Rc::new(v);
            }
        }
        unreachable!()
    }
}

struct Application {
    plugins: PluginList,
    logger: Box<dyn LogWriter>,
}

impl Application {
    fn new(plugins: PluginList) -> Self {
        let logging: Rc<&Logging> = plugins.logging();
        let ctx = logging.new_context("application".to_owned());
        let logger = Box::new(logging.new_logger(ctx));
        Application {
            plugins,
            logger,
        }
    }
    fn run(&mut self) {
        self.logger.log_raw("BEFORE run".to_owned());
        for mut plugin in self.plugins.all_mut() {
            Rc::<Plugin>::get_mut(&mut plugin).unwrap().run(&mut self.logger);
        }
        self.logger.log_raw("AFTER run".to_owned());
    }
    fn shutdown(&mut self) {
        self.logger.log_raw("BEFORE shutdown".to_owned());
        for mut plugin in self.plugins.all_rev_mut() {
            Rc::<Plugin>::get_mut(&mut plugin).unwrap().shutdown(&mut self.logger);
        }
        self.logger.log_raw("AFTER shutdown".to_owned());
    }
}
fn main() {
    let logging = Logging::new();
    let web = Web::new(&logging);
    let mut plugins = PluginList::new();
    plugins.register(Plugin::Logging(logging));
    plugins.register(Plugin::Web(web));
    let mut app = Application::new(plugins);
    app.run();
    app.shutdown();
}
