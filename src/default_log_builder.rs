extern crate log;
extern crate serde;
extern crate serde_json;
extern crate time;
extern crate uuid;

use log::Level;
use std::collections::BTreeMap;
use std::error::Error;
use serde_json::Value;

use log_builder::*;

pub struct DefaultLogBuilder<'a> {
    target: &'a str,
    level: &'a Level,
    name: Option<Value>,
    error: Option<&'a Error>,
    data: BTreeMap<&'a str, &'a erased_serde::Serialize>,
    context: BTreeMap<&'a str, &'a erased_serde::Serialize>,
}

#[derive(Serialize)]
struct LogEvent<'a> {
    data: &'a BTreeMap<&'a str, &'a erased_serde::Serialize>,
    context: &'a BTreeMap<&'a str, &'a erased_serde::Serialize>,
    time: String,
    level: String,
    id: &'a str,
    version: &'a str
}

impl <'a> LogBuilder<'a> for DefaultLogBuilder<'a> {
    fn set_event(&mut self, event: &'a str) -> &mut LogBuilder<'a> {
        self.name = Some(serde_json::to_value(&event.to_string()).unwrap());
        self
    }

    fn set_message (&mut self, message: &'a erased_serde::Serialize) -> &mut LogBuilder<'a> {
        self.data.insert("message", message);
        self
    }

    fn set_error(&mut self, error: &'a Error) -> &mut LogBuilder<'a> {
        self.error = Some(error);
        self
    }

    fn add_data(&mut self, key: &'a str, value: &'a erased_serde::Serialize) -> &mut LogBuilder<'a> {
        self.data.insert(key, value);
        self
    }

    fn add_context(&mut self, key: &'a str, value: &'a erased_serde::Serialize) -> &mut LogBuilder<'a> {
        self.context.insert(key, value);
        self
    }

    fn log(&mut self) {
        let now = time::now_utc();

        let event = LogEvent {
            id: &format!("{}", uuid::Uuid::new_v4().to_hyphenated()),
            time: format!("{}", now.rfc3339()),
            data: &self.data,
            context: &self.context,
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
        log!(target: self.target, *self.level, "{}", serialized);
    }
}

impl <'a> DefaultLogBuilder<'a> {
    pub fn new(target: &'a str, level: &'a Level) -> DefaultLogBuilder<'a> {
        DefaultLogBuilder {
            target,
            level: level,
            name: None,
            error: None,
            data: BTreeMap::new(),
            context: BTreeMap::new(),
        }
    }
}
