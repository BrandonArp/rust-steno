//#![feature(proc_macro)]

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate log;

extern crate serde;
extern crate serde_json;
extern crate erased_serde;

mod default_log_builder;
mod log_builder;
mod logger;
mod default_logger;
mod noop_log_builder;

pub use log_builder::*;
pub use default_log_builder::*;
pub use logger::*;
pub use default_logger::*;
pub use noop_log_builder::*;
