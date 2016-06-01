extern crate log;
extern crate serde;
extern crate serde_json;
extern crate time;
extern crate uuid;

use log::LogLevel;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::Display;
use serde::ser::Serialize;
use serde_json::Value;
use std::io::Write;

use log_builder::*;
use steno_serialize::*;

pub struct DefaultLogBuilder<'a> {
    target: &'a str,
    level: LogLevel,
    name: Option<Value>,
    error: Option<&'a Error>,
//    data: HashMap<&'a str, Value>,
    data: HashMap<&'a str, Box<Fn() -> Value + 'a>>,
    context: HashMap<&'a str, Value>,
}

#[derive(Serialize)]
struct LogEvent<'a> {
    data: &'a HashMap<&'a str, Value>,
    context: &'a HashMap<&'a str, Value>,
    time: String,
    level: String,
    id: &'a str,
    version: &'a str
}

impl <'a> LogBuilder<'a> for DefaultLogBuilder<'a> {
    fn set_event(&mut self, event: &'a str) -> &mut DefaultLogBuilder<'a> {
        self.name = Some(serde_json::to_value(&event.to_string()));
        self
    }

    fn set_message(&mut self, message: &'a str) -> &mut DefaultLogBuilder<'a> {
//        self.data.insert("message", serde_json::to_value(&message.to_string()));
        self.data.insert("message", Box::new( move || serde_json::to_value(message)));
        self
    }

    fn set_error(&mut self, error: &'a Error) -> &mut DefaultLogBuilder<'a> {
        self.error = Some(error);
        self
    }

    fn add_data<T>(&mut self, key: &'a str, value: &'a T) -> &mut DefaultLogBuilder<'a> where T: Serialize {
        self.data.insert(key, Box::new( move || serde_json::to_value(value)));
        self
    }

    fn add_context<T>(&mut self, key: &'a str, value: &'a T) -> &mut DefaultLogBuilder<'a> where T: Serialize {
        self.context.insert(key, serde_json::to_value(value));
        self
    }

    fn log(&mut self) {
        let now = time::now_utc();
        let mut serialized = HashMap::new();
        for (k, v) in self.data.iter() {
            serialized.insert(*k, v());
        }
        let event = LogEvent {
            id: &format!("{}", uuid::Uuid::new_v4().hyphenated()),
            time: format!("{}", now.rfc3339()),
            data: &serialized,
            context: &self.context,
            level: format!("{}", match self.level {
                LogLevel::Trace => "unknown",
                LogLevel::Debug => "debug",
                LogLevel::Info => "info",
                LogLevel::Warn => "warn",
                LogLevel::Error => "crit",
            }),
            version: "0"
        };
        log!(target: self.target, self.level, "{}", serde_json::to_string(&event).unwrap());
    }
}

impl <'a> DefaultLogBuilder<'a> {
    pub fn new(target: &'a str, level: &LogLevel) -> DefaultLogBuilder<'a> {
        DefaultLogBuilder {
            target: target,
            level: *level,
            name: None,
            error: None,
            data: HashMap::new(),
            context: HashMap::new(),
        }
    }
}
