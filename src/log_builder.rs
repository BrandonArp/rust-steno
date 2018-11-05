use std::error::Error;
use erased_serde::Serialize;

pub trait LogBuilder<'a> {
    fn set_event(&mut self, event: &'a str) -> &mut LogBuilder<'a>;
    fn set_message(&mut self, message: &'a erased_serde::Serialize) -> &mut LogBuilder<'a>;
    fn set_error(&mut self, error: &'a Error) -> &mut LogBuilder<'a>;
    fn add_data(&mut self, key: &'a str, value: &'a Serialize) -> &mut LogBuilder<'a>;
    fn add_context(&mut self, key: &'a str, value: &'a Serialize) -> &mut LogBuilder<'a>;
    fn log(&mut self) -> ();
}
