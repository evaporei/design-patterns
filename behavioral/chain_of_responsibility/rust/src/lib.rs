#[derive(Debug, PartialOrd, PartialEq)]
pub enum Level {
    Info = 1,
    Debug = 2,
    Error = 3,
}

pub trait AbstractLogger {
    fn set_next_logger(&mut self, abstract_logger: Box<AbstractLogger>);
    fn level(&self) -> &Level;
    fn next_logger(&self) -> &Option<Box<AbstractLogger>>;
    fn log_message(&self, level: Level, message: &String) {
        if self.level() <= &level {
            self.write(&message);
        }
        if let Some(next_logger) = self.next_logger() {
            next_logger.log_message(level, &message);
        }
    }
    fn write(&self, message: &String);
}

pub struct ConsoleLogger {
    level: Level,
    next_logger: Option<Box<AbstractLogger>>,
}

impl ConsoleLogger {
    pub fn new(level: Level) -> ConsoleLogger {
        ConsoleLogger { level, next_logger: None }
    }
}

impl AbstractLogger for ConsoleLogger {
    fn set_next_logger(&mut self, abstract_logger: Box<AbstractLogger>) {
        self.next_logger = Some(abstract_logger);
    }
    fn level(&self) -> &Level {
        &self.level
    }
    fn next_logger(&self) -> &Option<Box<AbstractLogger>> {
        &self.next_logger
    }
    fn write(&self, message: &String) {
        println!("Standard Console::Logger: {}", message);
    }
}

pub struct ErrorLogger {
    level: Level,
    next_logger: Option<Box<AbstractLogger>>,
}

impl ErrorLogger {
    pub fn new(level: Level) -> ErrorLogger {
        ErrorLogger { level, next_logger: None }
    }
}

impl AbstractLogger for ErrorLogger {
    fn set_next_logger(&mut self, abstract_logger: Box<AbstractLogger>) {
        self.next_logger = Some(abstract_logger);
    }
    fn level(&self) -> &Level {
        &self.level
    }
    fn next_logger(&self) -> &Option<Box<AbstractLogger>> {
        &self.next_logger
    }
    fn write(&self, message: &String) {
        println!("Error Console::Logger: {}", message);
    }
}

pub struct FileLogger {
    level: Level,
    next_logger: Option<Box<AbstractLogger>>,
}

impl FileLogger {
    pub fn new(level: Level) -> FileLogger {
        FileLogger { level, next_logger: None }
    }
}

impl AbstractLogger for FileLogger {
    fn set_next_logger(&mut self, abstract_logger: Box<AbstractLogger>) {
        self.next_logger = Some(abstract_logger);
    }
    fn level(&self) -> &Level {
        &self.level
    }
    fn next_logger(&self) -> &Option<Box<AbstractLogger>> {
        &self.next_logger
    }
    fn write(&self, message: &String) {
        println!("File::Logger: {}", message);
    }
}
