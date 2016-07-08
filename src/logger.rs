use log_builder::LogBuilder;

pub trait Logger<'a> {
    fn info(&self) -> Box<LogBuilder<'a>>;
}
