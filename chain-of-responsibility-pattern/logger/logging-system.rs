trait Logger {
    fn set_next(&mut self, next: Box<dyn Logger>) -> &mut dyn Logger;
    fn log(&self, level: &str, message: &str);
}

struct BaseLogger {
    log_level: String,
    next_handler: Option<Box<dyn Logger>>,
}
impl BaseLogger {
    fn new(log_level: &str) -> Self {
        Self {
            log_level: log_level.to_string(),
            next_handler: None,
        }
    }
}

impl Logger for BaseLogger {
    fn set_next(&mut self, next: Box<dyn Logger>) -> &mut dyn Logger {
        self.next_handler = Some(next);
        // self.next_handler.as_mut().unwrap().as_mut()

        // as_deref_mut() converts Option<Box<T>> into Option<&mut T>
        self.next_handler.as_deref_mut().unwrap()
    }

    fn log(&self, level: &str, message: &str) {
        if self.log_level == level {
            println!("{}: {}", level, message);
        }

        if let Some(next_handler) = &self.next_handler {
            next_handler.log(level, message);
        }
    }
}

// Concrete loggers
// struct InfoLogger(BaseLogger);
// struct DebugLogger(BaseLogger);
// struct ErrorLogger(BaseLogger);

fn main() {
    let mut info_logger = Box::new(BaseLogger::new("INFO")) as Box<dyn Logger>;
    let debug_logger = Box::new(BaseLogger::new("DEBUG")) as Box<dyn Logger>;
    let error_logger = Box::new(BaseLogger::new("ERROR")) as Box<dyn Logger>;

    // Link the loggers
    info_logger.set_next(debug_logger).set_next(error_logger);

    // Using the chain of loggers
    info_logger.log("INFO", "This is an info message");
    info_logger.log("DEBUG", "This is a debug message");
    info_logger.log("ERROR", "This is an error message");
}
