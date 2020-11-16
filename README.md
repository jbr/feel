# feel: it goes beyond touch

a microscopic cli tool that is like touch, but also does mkdir -p on
the directory base before touching the file

```bash
$ feel path/that/doesnt/exist/file
```

## Installation

```sh
$ cargo install feel
```

<!-- * [CI ![CI][ci-badge]][ci] -->
* [Releases][releases] [![crates.io version][version-badge]][lib-rs]
* [Contributing][contributing]

<!-- [ci]: https://github.com/jbr/feel/actions?query=workflow%3ACI -->
<!-- [ci-badge]: https://github.com/jbr/feel/workflows/CI/badge.svg -->
[releases]: https://github.com/jbr/feel/releases
[contributing]: https://github.com/jbr/feel/blob/master/.github/CONTRIBUTING.md
[lib-rs]: https://lib.rs/feel
[version-badge]: https://img.shields.io/crates/v/feel.svg?style=flat-square

## Safety
This crate uses ``#![forbid(unsafe_code)]`` to ensure everything is implemented in
100% Safe Rust.

## License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br/>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
