extern crate serde;

use log_builder::LogBuilder;

use std::error::Error;
use serde::Serialize;

pub struct NoopLogBuilder { }

impl <'a> LogBuilder<'a> for NoopLogBuilder {
    fn set_event(&mut self, event: &'a str) -> &mut NoopLogBuilder {
        self
    }

    fn set_message(&mut self, message: &'a str) -> &mut NoopLogBuilder {
        self
    }

    fn set_error(&mut self, error: &'a Error) -> &mut NoopLogBuilder {
        self
    }

    fn add_data<T>(&mut self, key: &'a str, value: &'a T) -> &mut NoopLogBuilder where T: Serialize {
        self
    }

    fn add_context<T>(&mut self, key: &'a str, value: &'a T) -> &mut NoopLogBuilder where T: Serialize {
        self
    }

    fn log(&mut self) { }
}

impl NoopLogBuilder {
    pub fn new() -> NoopLogBuilder {
        NoopLogBuilder { }
    }
}
