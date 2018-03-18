#[macro_use]
extern crate slog;
extern crate slog_conf;

extern crate toml;

use slog::Drain;

use slog_conf::{Config, TermConfig};


fn main() {
    // load a configuration with the default deserializers
    let config: Box<Config> = toml::from_str(include_str!("term.toml")).unwrap();

    // show what we have just loaded
    println!("-- TermConfig ------------");
    println!("{:#?}\n", config.downcast_ref::<TermConfig>().unwrap());

    // show it again, this time as toml output (via serialization)
    // Note: we have to use an intermediate `toml::Value` here because TOML
    // requires that tables (i.e. maps, structs, or sequences) must be emitted
    // last. We can acheive this with an intermediate `toml::Value`. This
    // should not be necessary for other formats.
    let val = toml::Value::try_from(&*config).unwrap();
    let out = toml::to_string_pretty(&val).unwrap();
    println!("-- TermConfig as TOML ----");
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
