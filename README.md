# git-appraise (Rust) [![travis-badge][]][travis] [![downloads-badge][] ![release-badge][]][crate] [![license-badge][]](#license)

This is a Rust port of the [git-appraise][] library (written in Go). Currently
it offers read-only access to the stored reviews in a repository via the
[git2-rs][] library.  Eventually it may be expanded to include mutation and
creation of new reviews.

[travis-badge]: https://img.shields.io/travis/Nemo157/git-appraise-rs/master.svg?style=flat-square
[downloads-badge]: https://img.shields.io/crates/d/git-appraise.svg?style=flat-square
[release-badge]: https://img.shields.io/crates/v/git-appraise.svg?style=flat-square
[license-badge]: https://img.shields.io/crates/l/git-appraise.svg?style=flat-square
[travis]: https://travis-ci.org/Nemo157/git-appraise-rs
[crate]: https://crates.io/crates/git-appraise

[git-appraise]: https://github.com/google/git-appraise
[git2-rs]: https://github.com/alexcrichton/git2-rs

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you shall be dual licensed as above, without any
additional terms or conditions.

## Developing

If building on OS X with a `homebrew` installed copy of OpenSSL you'll need to
specify where this is to enable building `libssh2-sys`.  Use something like:

```sh
OPENSSL_ROOT_DIR=`brew --prefix openssl` \
OPENSSL_LIB_DIR=`brew --prefix openssl`/lib \
OPENSSL_INCLUDE_DIR=`brew --prefix openssl`/include \
cargo build
```

This library uses the combined `serde_macros` on nightly and `syntex` on stable
approach described [in the `serde` readme][serde-readme]. To build on stable
just use `cargo build` as normal, to build on nightly use

```sh
cargo build --no-default-features --features nightly
```

It's recommended to use nightly for development as the error messages should be
better.

[serde-readme]: https://github.com/serde-rs/serde#using-serde-with-stable-rust-syntex-and-serde_codegen
