use super::logging;
use crate::Plugin;

pub struct Cli {
    logger: Box<dyn logging::LogWriter>,
}

impl Cli {
    pub fn new(logging: &logging::Logging) -> Self {
        let ctx = logging.new_context("cli".to_owned());
        let logger = Box::new(logging.new_logger(ctx));
        Self {
            logger,
        }
    }
}

impl Plugin for Cli {
    fn run(&mut self) {
        self.logger.log_raw("run".to_owned());
    }
    fn shutdown(&mut self) {
        self.logger.log_raw("shutdown".to_owned());
    }
}

