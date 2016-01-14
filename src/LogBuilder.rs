use std::error::Error;
use std::fmt::Display;

pub trait LogBuilder<'a> {
    fn set_event(&mut self, event: String) -> &mut Self;
    fn set_message(&mut self, message: String) -> &mut Self;
    fn set_error(&mut self, error: &'a Error) -> &mut Self;
    fn add_data<T>(&mut self, key: String, value: &'a T) -> &mut Self where T: Display;
    fn add_context<T>(&mut self, key: String, value: &'a T) -> &mut Self where T: Display;
}
