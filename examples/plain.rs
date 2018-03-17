#[macro_use]
extern crate slog;
extern crate slog_conf;

extern crate toml;

use slog::Drain;

use slog_conf::{Config, PlainConfig};


fn main() {
    // load a configuration with the default deserializers
    let config: Box<Config> = toml::from_str(include_str!("plain.toml")).unwrap();

    // show what we have just loaded
    println!("-- PlainConfig ------------");
    println!("{:#?}\n", config.downcast_ref::<PlainConfig>().unwrap());

    // show it again, this time as toml output (via serialization)
    let val = toml::Value::try_from(&*config).unwrap();
    let out = toml::to_string_pretty(&val).unwrap();
    println!("-- PlainConfig as TOML ----");
    println!("{}\n", out);

    // build a logger
    let (async, _guard) = slog_conf::build(config.as_ref()).unwrap();
    let log = slog::Logger::root(async.fuse(), o!());

    // use the logger
    warn!(log, "a warning"; "a" => "b");
    info!(log, "some information that might be relevant"; "b" => "c");

    let log = log.new(o!("a" => "b"));
    debug!(log, "a debug message"; "f" => "g");
    debug!(log, "another debug message");
    error!(log, "an error message");
}
