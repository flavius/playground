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