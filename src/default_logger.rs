extern crate log;

use logger::Logger;
use log_builder::LogBuilder;
use default_log_builder::DefaultLogBuilder;
use noop_log_builder::NoopLogBuilder;

use log::LogLevel;

pub struct DefaultLogger<'a> {
    target: &'a str
}

impl <'a> Logger<'a> for DefaultLogger<'a> {
    fn info(&self) -> DefaultLogBuilder<'a>  {
        self.new_for_level(&LogLevel::Info)
    }
}

impl<'a> DefaultLogger<'a> {
    pub fn new(target: &'a str) -> DefaultLogger<'a> {
        DefaultLogger {
            target: target
        }
    }

    fn new_for_level(&self, level: &LogLevel) -> DefaultLogBuilder<'a> {
        DefaultLogBuilder::new(self.target, level)
    }
}
