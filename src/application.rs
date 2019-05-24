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

