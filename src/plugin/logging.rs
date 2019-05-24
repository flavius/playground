use crate::Plugin;

pub struct Logging {
}

impl Logging {
    pub fn new() -> Self {
        Self {
        }
    }
    pub fn new_context(&self, id: String) -> LoggingContext {
        LoggingContext(id)
    }
    pub fn new_logger(&self, context: LoggingContext) -> impl LogWriter {
        InMemoryLogger::new(context)
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

pub trait LogWriter {
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

pub struct LoggingContext(String);
