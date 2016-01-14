use std::collections::HashMap;
use std::error::Error;
use std::fmt::Display;

use logbuilder;

pub struct DefaultLogBuilder<'a> {
    event: Option<String>,
    message: Option<String>,
    error: Option<&'a Error>,
    data: HashMap<String, &'a Display>,
    context: HashMap<String, &'a Display>,
}

impl <'a> logbuilder::LogBuilder<'a> for DefaultLogBuilder<'a> {
    fn set_event(&mut self, event: String) -> &mut DefaultLogBuilder<'a> {
        self.event = Some(event);
        self
    }

    fn set_message(&mut self, message: String) -> &mut DefaultLogBuilder<'a> {
        self.message = Some(message);
        self
    }

    fn set_error(&mut self, error: &'a Error) -> &mut DefaultLogBuilder<'a> {
        self.error = Some(error);
        self
    }

    fn add_data<T>(&mut self, key: String, value: &'a T) -> &mut DefaultLogBuilder<'a> where T: Display {
        self.data.insert(key, value);
        self
    }

    fn add_context<T>(&mut self, key: String, value: &'a T) -> &mut DefaultLogBuilder<'a> where T: Display {
        self.context.insert(key, value);
        self
    }
}

impl <'a> DefaultLogBuilder<'a> {
    pub fn new() -> DefaultLogBuilder<'a> {
        DefaultLogBuilder {
            event: None,
            message: None,
            error: None,
            data: HashMap::new(),
            context: HashMap::new()
        }
    }
}
