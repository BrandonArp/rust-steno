extern crate log;

use logger::Logger;
use log_builder::LogBuilder;
use default_log_builder::DefaultLogBuilder;
use noop_log_builder::NoopLogBuilder;

use log::Level;

pub struct DefaultLogger<'a> {
    target: &'a str
}

impl <'a> Logger<'a> for DefaultLogger<'a> {
    fn info(&self) -> Box<'a + LogBuilder<'a>>  {
        self.parameterized_new_builder(&Level::Info)
    }

    fn warn(&self) -> Box<'a + LogBuilder<'a>>  {
        self.parameterized_new_builder(&Level::Warn)
    }

    fn error(&self) -> Box<'a + LogBuilder<'a>>  {
        self.parameterized_new_builder(&Level::Debug)
    }

    fn debug(&self) -> Box<'a + LogBuilder<'a>>  {
        self.parameterized_new_builder(&Level::Debug)
    }

    fn trace(&self) -> Box<'a + LogBuilder<'a>>  {
        self.parameterized_new_builder(&Level::Debug)
    }

}

impl<'a> DefaultLogger<'a> {
    pub fn new(target: &'a str) -> DefaultLogger<'a> {
        DefaultLogger {
            target: target
        }
    }

    fn parameterized_new_builder(&self, level: &'a Level) -> Box<'a + LogBuilder<'a>> {
        if log_enabled!(target: self.target, Level::Info) {
            Box::new(DefaultLogBuilder::new(self.target, level))
        } else {
            Box::new(NoopLogBuilder::new())
        }
    }
}
