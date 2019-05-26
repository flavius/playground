use super::logging;
use crate::Plugin;

pub struct Appendlog {
    logger: Box<dyn logging::LogWriter>,
}

impl Appendlog {
    pub fn new(logging: &logging::Logging) -> Self {
        let ctx = logging.new_context("appendlog".to_owned());
        let logger = Box::new(logging.new_logger(ctx));
        Self {
            logger,
        }
    }
}

impl Plugin for Appendlog {
    fn run(&mut self) {
        self.logger.log_raw("run".to_owned());
    }
    fn shutdown(&mut self) {
        self.logger.log_raw("shutdown".to_owned());
    }
}

