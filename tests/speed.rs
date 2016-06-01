extern crate log;
extern crate steno;
extern crate log4rs;
extern crate time;
extern crate uuid;
extern crate fastlog;

use steno::DefaultLogBuilder;
use steno::LogBuilder;
use time::PreciseTime;
use std::io::{Error, ErrorKind, Write};

#[test]
fn speedy() {
    //let iterations = 1_000_000;
    let iterations = 1;
    let nanos_per_second = 1_000_000_000;
    std::fs::remove_file("test.log").ok();
    //fastlog::LogBuilder::new().build().unwrap().init().unwrap();
    log4rs::init_file("tests/logger.yaml", Default::default()).unwrap();
    let start = PreciseTime::now();
    let ref uuid1 = format!("{}", uuid::Uuid::new_v4().hyphenated());
    let ref uuid2 = format!("{}", uuid::Uuid::new_v4().hyphenated());
    let ref error = Error::new(ErrorKind::Other, "oh no!");

    for _ in 0..iterations {
//        let mut lb = DefaultLogBuilder::new("test::logger", &log::LogLevel::Info);
        {
            let mut lb = DefaultLogBuilder::new("test::logger", &log::LogLevel::Info);
            //        lb.add_context("requestId", uuid1);
            //        lb.add_data("userId", uuid2);
            lb.add_context("requestId", uuid1);
            lb.add_data("userId", uuid2)
            .set_message("test message")
            .set_error(error)
            .log();
        }
    }
    let stop = PreciseTime::now();
    let duration = start.to(stop);
    let speed = (iterations as f64 / duration.num_nanoseconds().unwrap() as f64) * nanos_per_second as f64;
    writeln!(&mut std::io::stderr(), "{} writes/second", speed).unwrap();
}
