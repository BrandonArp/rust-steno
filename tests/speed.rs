extern crate log;
extern crate steno;
extern crate time;
extern crate uuid;
extern crate slog;
extern crate slog_stdlog;
extern crate slog_scope;

use steno::DefaultLogger;
use steno::Logger;
use time::PreciseTime;
use std::io::{Error, ErrorKind, Write};
use slog::*;

#[test]
fn speedy() {
    let drain = slog::Discard;
    let logger = slog::Logger::root(drain, o!());
    let _scope_guard = slog_scope::set_global_logger(logger);
    let _log_guard = slog_stdlog::init().unwrap();

    let nanos_per_second = 1_000_000_000;
    std::fs::remove_file("test.log").ok();
    let uuid1 = format!("{}", uuid::Uuid::new_v4().to_hyphenated());
    let uuid2 = format!("{}", uuid::Uuid::new_v4().to_hyphenated());
    let error = Error::new(ErrorKind::Other, "oh no!");
    let logger = DefaultLogger::new("test::logger");

    writeln!(&mut std::io::stderr(), "Starting benchmark to determine rough speed");
    let mut bench_iterations = 1024;
    loop {
        let start_warmup = PreciseTime::now();
        for _ in 0..bench_iterations {
            logger.info()
                .add_context("requestId", &uuid1)
                .add_data("userId", &uuid2)
                .add_data("me", &"too")
                .set_message(&"test message")
                .set_error(&error)
                .log();
        }
        let end_warmup = PreciseTime::now();
        if start_warmup.to(end_warmup).num_nanoseconds().unwrap() > nanos_per_second {
            break;
        }
        bench_iterations <<= 1;
    }


    let iterations = bench_iterations * 5;
    writeln!(&mut std::io::stderr(), "Determined rough speed, running full benchmark with {} iterations", iterations);
    let start = PreciseTime::now();
    for _ in 0..iterations {
        let mut builder = logger.info();
        builder
          .add_context("requestId", &uuid1)
          .add_data("userId", &uuid2)
          .set_message(&"test message")
          .set_error(&error)
          .log();
    }
    let stop = PreciseTime::now();
    let duration = start.to(stop);
    let speed = (iterations as f64 / duration.num_nanoseconds().unwrap() as f64) * nanos_per_second as f64;
    writeln!(&mut std::io::stderr(), "{:.0} writes/second", speed).unwrap();
}
