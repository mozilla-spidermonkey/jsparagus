[![Rust][Rust Badge]][Rust CI Link]
[![NotImplemented Counter][NotImplemented Badge]][NotImplemented Search]
[![Fuzzbug days since][Fuzzbug Days Badge]][Fuzzbugs]
[![Fuzzbug open][Fuzzbug Open Badge]][Fuzzbugs]


# jsparagus - A JavaScript parser written in Rust

jsparagus is intended to replace the JavaScript parser in Firefox.

Current status:

*   jsparagus is not on crates.io yet. The AST design is not stable
    enough.  We do have a build of the JS shell that includes jsparagus
    as an option (falling back on C++ for features jsparagus doesn't
    support). See
    [mozilla-spidermonkey/rust-frontend](https://github.com/mozilla-spidermonkey/rust-frontend).

*   It can parse a lot of JS scripts, and will eventually be able to parse everything.
    See the current limitations below, or our GitHub issues.

*   Our immediate goal is to [support parsing everything in Mozilla's JS
    test suite and the features in test262 that Firefox already
    supports](https://github.com/mozilla-spidermonkey/jsparagus/milestone/1).

Join us on Discord: https://discord.gg/tUFFk9Y


## Getting started

```sh
cargo install cargo-fmt
git config core.hooksPath .githooks
make init
make all
```

(**Note:** This takes about 3 minutes to run on my laptop. Part of
jsparagus is generated by a Python script, and the script is pretty
slow. We're working on it!)

When it's done, you can:

*   Run `make check` to make sure things are working.

*   `cd rust && cargo run` to test the JS parser and bytecode emitter.


## Limitations

It's *all* limitations, but I'll try to list the ones that are relevant
to parsing JS.

*   Lookahead assertions are limited to one token. (The JS grammar
    contains an occasional
    ``[lookahead != `let [`]``
    and even
    ``[lookahead != `async [no LineTerminator here] function`]``.)

*   Error messages are poor.

*   No table compaction or table optimization. There's plenty of
    low-hanging fruit there.


[Rust Badge]: https://github.com/mozilla-spidermonkey/jsparagus/workflows/Rust/badge.svg
[Rust CI Link]: https://github.com/mozilla-spidermonkey/jsparagus/actions?query=branch%3Amaster
[NotImplemented Badge]: https://img.shields.io/endpoint?url=https%3A%2F%2Fraw.githubusercontent.com%2Fcodehag%2Fjsparagus%2Fci_results%2F.metrics%2Fbadges%2Fnot-implemented.json
[NotImplemented Search]: https://github.com/mozilla-spidermonkey/jsparagus/search?q=notimplemented&unscoped_q=notimplemented
[Fuzzbug days Badge]: https://img.shields.io/endpoint?url=https%3A%2F%2Fraw.githubusercontent.com%2Fcodehag%2Fjsparagus%2Fci_results%2F.metrics%2Fbadges%2Fsince-last-fuzzbug.json
[Fuzzbug Open Badge]: https://img.shields.io/endpoint?url=https%3A%2F%2Fraw.githubusercontent.com%2Fcodehag%2Fjsparagus%2Fci_results%2F.metrics%2Fbadges%2Fopen-fuzzbug.json
[Fuzzbugs]: https://github.com/mozilla-spidermonkey/jsparagus/issues?utf8=%E2%9C%93&q=label%3AlibFuzzer+
