extern crate serde;

use log_builder::LogBuilder;

use std::error::Error;
use erased_serde::Serialize;

#[derive(Default)]
pub struct NoopLogBuilder { }

impl <'a> LogBuilder<'a> for NoopLogBuilder {
    fn set_event(&mut self, _event: &'a str) -> &mut LogBuilder<'a> {
        self
    }

    fn set_message(&mut self, _message: &'a erased_serde::Serialize) -> &mut LogBuilder<'a> {
        self
    }

    fn set_error(&mut self, _error: &'a Error) -> &mut LogBuilder<'a> {
        self
    }

    fn add_data(&mut self, _key: &'a str, _value: &'a Serialize) -> &mut LogBuilder<'a> {
        self
    }

    fn add_context(&mut self, _key: &'a str, _value: &'a Serialize) -> &mut LogBuilder<'a> {
        self
    }

    fn log(&mut self) { }
}

impl NoopLogBuilder {
    pub fn new() -> NoopLogBuilder {
        NoopLogBuilder { }
    }
}
