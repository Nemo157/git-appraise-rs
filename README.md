git-appraise-rs
===============

Developing
----------

If building on OS X with a `homebrew` installed copy of OpenSSL you'll need to
specify where this is to enable building `libssh2-sys` and `openssl-sys-extras`.
Use something like:

```sh
OPENSSL_ROOT_DIR=/usr/local/Cellar/openssl/1.0.2d_1 \
DEP_OPENSSL_INCLUDE=/usr/local/Cellar/openssl/1.0.2d_1/include \
cargo build
```
