use log_builder::LogBuilder;
use default_log_builder::DefaultLogBuilder;

pub trait Logger<'a> {
    fn info(&self) -> DefaultLogBuilder<'a>;
}
