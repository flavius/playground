use super::logging;
use super::appendlog;
use crate::Plugin;

pub struct Projector {
    logger: Box<dyn logging::LogWriter>,
}

impl Projector {
    pub fn new(appendlog: &appendlog::Appendlog, logging: &logging::Logging) -> Self {
        let ctx = logging.new_context("projector".to_owned());
        let logger = Box::new(logging.new_logger(ctx));
        Self {
            logger,
        }
    }
}

impl Plugin for Projector {
    fn run(&mut self) {
        self.logger.log_raw("run".to_owned());
    }
    fn shutdown(&mut self) {
        self.logger.log_raw("shutdown".to_owned());
    }
}
