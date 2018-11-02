extern crate log;
extern crate serde;
extern crate serde_json;
extern crate time;
extern crate uuid;

use std;
use log::Level;
use std::borrow::Borrow;
use std::collections::BTreeMap;
use std::error::Error;
use std::fmt::Display;
use serde::ser::Serialize;
use serde_json::Value;
use std::io::Write;

use log_builder::*;
use steno_serialize::*;

pub struct DefaultLogBuilder<'a> {
    target: &'a str,
    level: Level,
    name: Option<Value>,
    error: Option<&'a Error>,
//    data: HashMap<&'a str, Value>,
    data: BTreeMap<&'a str, Box<Fn() -> Value + 'a>>,
    context: BTreeMap<&'a str, Box<Fn() -> Value + 'a>>,
}

#[derive(Serialize)]
struct LogEvent<'a> {
    data: &'a BTreeMap<&'a str, Value>,
    context: &'a BTreeMap<&'a str, Value>,
    time: String,
    level: String,
    id: &'a str,
    version: &'a str
}

impl <'a> LogBuilder<'a> for DefaultLogBuilder<'a> {
    fn set_event(&mut self, event: &'a str) -> &mut DefaultLogBuilder<'a> {
        self.name = Some(serde_json::to_value(&event.to_string()).unwrap());
        self
    }

    fn set_message(&mut self, message: &'a str) -> &mut DefaultLogBuilder<'a> {
//        self.data.insert("message", serde_json::to_value(&message.to_string()));
        self.data.insert("message", Box::new( move || serde_json::to_value(message).unwrap()));
        self
    }

    fn set_error(&mut self, error: &'a Error) -> &mut DefaultLogBuilder<'a> {
        self.error = Some(error);
        self
    }

    fn add_data<T>(&mut self, key: &'a str, value: &'a T) -> &mut DefaultLogBuilder<'a> where T: Serialize {
        self.data.insert(key, Box::new( move || serde_json::to_value(value).unwrap()));
        self
    }

    fn add_context<T>(&mut self, key: &'a str, value: &'a T) -> &mut DefaultLogBuilder<'a> where T: Serialize {
        self.context.insert(key, Box::new( move || serde_json::to_value(value).unwrap()));
        self
    }

    fn log(&mut self) {
        if log_enabled!(target: self.target, self.level) {
            let now = time::now_utc();
            let mut serialized_data = BTreeMap::new();
            for (k, v) in self.data.iter() {
                serialized_data.insert(*k, v());
            }
            let mut serialized_context = BTreeMap::new();
            for (k,v) in self.context.iter() {
                serialized_context.insert(*k, v());
            }

            let event = LogEvent {
                id: &format!("{}", uuid::Uuid::new_v4().to_hyphenated()),
                time: format!("{}", now.rfc3339()),
                data: &serialized_data,
                context: &serialized_context,
                level: format!("{}", match self.level {
                    Level::Trace => "unknown",
                    Level::Debug => "debug",
                    Level::Info => "info",
                    Level::Warn => "warn",
                    Level::Error => "crit",
                }),
                version: "0"
            };

            let serialized = serde_json::to_string(&event).unwrap();
            log!(target: self.target, self.level, "{}", serialized);
        }
    }
}

impl <'a> DefaultLogBuilder<'a> {
    pub fn new(target: &'a str, level: &Level) -> DefaultLogBuilder<'a> {
        DefaultLogBuilder {
            target: target,
            level: *level,
            name: None,
            error: None,
            data: BTreeMap::new(),
            context: BTreeMap::new(),
        }
    }
}
