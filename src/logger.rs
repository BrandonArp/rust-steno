use log_builder::LogBuilder;

pub trait Logger<'a> {
    fn info(&self) -> Box<'a + LogBuilder<'a>>;
    fn warn(&self) -> Box<'a + LogBuilder<'a>>;
    fn error(&self) -> Box<'a + LogBuilder<'a>>;
    fn debug(&self) -> Box<'a + LogBuilder<'a>>;
    fn trace(&self) -> Box<'a + LogBuilder<'a>>;
}
