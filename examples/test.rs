#[macro_use]
extern crate slog;
extern crate hostname;
extern crate slog_async;
extern crate slog_gelf;

use slog::Drain;

fn main() {
    let hostname = hostname::get_hostname().unwrap();

    let drain = slog_gelf::Gelf::new(&hostname, "192.168.0.1011:12201")
        .unwrap()
        .fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    let log = slog::Logger::root(drain, o!("key" => "value"));

    info!(log,
        "An example log message";
        "k1" => "v1",
        "k2" => "v2",
    );
}
