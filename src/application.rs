use crate::plugin;

pub struct Application {
    plugins: plugin::PluginList,
    logger: Box<dyn plugin::logging::LogWriter>,
}

impl Application {
    pub fn new(mut plugins: plugin::PluginList) -> Option<Self> {
        let logging = plugins.get_plugin::<plugin::Logging>();
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
    pub fn run(&mut self) {
        //self.logger.log_raw("BEFORE run".to_owned());
        for plugin in &mut *self.plugins {
            plugin.run(&mut self.logger);
        }
        //self.logger.log_raw("AFTER run".to_owned());
    }
    pub fn shutdown(&mut self) {
        //self.logger.log_raw("BEFORE shutdown".to_owned());
        for plugin in self.plugins.iter_rev_mut() {
            plugin.shutdown(&mut self.logger);
        }
        //self.logger.log_raw("AFTER shutdown".to_owned());
    }
}

