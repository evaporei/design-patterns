extern crate chain_of_responsibility;

use chain_of_responsibility::*;

fn main() {
    let logger_chain = get_chain_of_loggers();

    logger_chain.log_message(Level::Info, &"This is an information.".to_string());
    logger_chain.log_message(Level::Debug, &"This is an debug level information.".to_string());
    logger_chain.log_message(Level::Error, &"This is an error information.".to_string());
}

fn get_chain_of_loggers() -> Box<AbstractLogger> {
    let mut error_logger = ErrorLogger::new(Level::Error);
    let mut file_logger = FileLogger::new(Level::Debug);
    let console_logger = ConsoleLogger::new(Level::Info);

    file_logger.set_next_logger(Box::new(console_logger));
    error_logger.set_next_logger(Box::new(file_logger));

    Box::new(error_logger)
}
