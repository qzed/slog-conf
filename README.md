# Slog Conf

[![Build Status]][travis]

[Build Status]: https://api.travis-ci.org/qzed/slog-conf.svg?branch=master
[travis]: https://travis-ci.org/qzed/slog-conf

Highly customizable runtime-configuration for [`slog-rs/slog`][slog] and its backends with opinionated defaults.

- [API Documentation][doc]

This crate can be used to create and configure a `slog` logger using configuration files.
It takes care of serialization, deserialization, and creation of `Drain`s and `Logger`s.
While the defaults provided by this crate are somewhat opinionated (i.e. always creating an `Async` drain using [`slog_async`][slog-async]), all of these default implementations can be replaced by your custom implementations without too much of a hassle.
Have a look at the API-documentation for more detail, or the examples directory for some examples.

## Currently under development

This crate is in a prototype-state and is thus not (yet) on [`crates.io`](https://crates.io).

If you still want to use this crate, simply add

```toml
slog-conf = { git = "https://github.com/qzed/slog-conf", branch = "master" }
```

to the `[dependencies]` section in your `Cargo.toml` file.
There are no guarantees for stability, you may however want to choose a specific `tag` instead of a `branch` if this is your concern.

## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

[slog]: https://github.com/slog-rs/slog
[slog-async]: https://github.com/slog-rs/async
[doc]: https://qzed.github.io/slog-conf/slog_conf
