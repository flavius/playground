use std::any::{Any, TypeId};
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
    //TODO: remove?
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

struct LoggingContext(String);
