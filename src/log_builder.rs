use std::borrow::Borrow;
use std::error::Error;
use std::fmt::Display;
use serde::ser::Serialize;

pub trait LogBuilder<'a> {
    fn set_event(&mut self, event: &'a str) -> &mut Self;
    fn set_message(&mut self, message: &'a str) -> &mut Self;
    fn set_error(&mut self, error: &'a Error) -> &mut Self;
    fn add_data<T>(&mut self, key: &'a str, value: &'a T) -> &mut Self where T: Serialize;
    fn add_context<T>(&mut self, key: &'a str, value: &'a T) -> &mut Self where T: Serialize;
    fn log(self) -> ();
}
