# git-appraise (Rust) [![release-badge][]][cargo]

This is a Rust port of the [git-appraise][] library (written in Go). Currently
it offers read-only access to the stored reviews in a repository via the
[git2-rs][] library.  Eventually it may be expanded to include mutation and
creation of new reviews.

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you shall be dual licensed as above, without any
additional terms or conditions.

[release-badge]: https://img.shields.io/badge/crate-coming--soon-yellowgreen.svg?style=flat-square
[cargo]: https://crates.io/crates/git-appraise
[git-appraise]: https://github.com/google/git-appraise
[git2-rs]: https://github.com/alexcrichton/git2-rs

## Developing

If building on OS X with a `homebrew` installed copy of OpenSSL you'll need to
specify where this is to enable building `libssh2-sys`.  Use something like:

```sh
OPENSSL_ROOT_DIR=/usr/local/Cellar/openssl/1.0.2d_1 \
DEP_OPENSSL_INCLUDE=/usr/local/Cellar/openssl/1.0.2d_1/include \
cargo build
```
