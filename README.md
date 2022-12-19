Rust Pratice
================================================================================

Rust言語の練習

# 公式ドキュメント

- [The Rust Programming Language](https://doc.rust-lang.org/)
  - [book](https://doc.rust-lang.org/book/)
- [The Rust Programming Language 日本語版](https://doc.rust-jp.rs/)
  - [book](https://doc.rust-jp.rs/book-ja/)
- [The Cargo Book](https://doc.rust-lang.org/cargo/)
- [creates.io: Rust Package Registry](https://crates.io/)
- [rustup.rs - The Rust toolchain installer](https://rustup.rs/)

# よく使うコマンドまとめ

## Setup Toolchain
```
$ rustup toolchain install stable

$ rustup default stable

$ rustup toolchain list
stable-x86_64-unknown-linux-gnu (default)

$ rustup show
Default host: x86_64-unknown-linux-gnu
rustup home:  /home/user/.rustup

installed toolchains
--------------------

stable-x86_64-unknown-linux-gnu (default)
nightly-2022-01-14-x86_64-unknown-linux-gnu
nightly-x86_64-unknown-linux-gnu

active toolchain
----------------

stable-x86_64-unknown-linux-gnu (default)
rustc 1.65.0 (897e37553 2022-11-02)
```

## Start Project
```
$ cargo new hello_world
$ cd hello_world

$ cargo build
   Compiling hello_world v0.1.0 (file:///path/to/package/hello_world)

$ ./target/debug/hello_world
Hello, world!

$ cargo run
     Fresh hello_world v0.1.0 (file:///path/to/package/hello_world)
   Running `target/hello_world`
Hello, world!
```

### gitignore

```
*/target
**/*.rs.bk
```

### その他参考
--------------------------------------------------------------------------------

- [Rust Documentation](https://doc.rust-lang.org/)
    - [Rust Programming Language](https://doc.rust-lang.org/book/)
    - [The Rust Reference](https://doc.rust-lang.org/reference.html)
    - [Standard Library API Reference](https://doc.rust-lang.org/std/)
    - [The Rustonomicon](https://doc.rust-lang.org/nomicon/)
    - [Compiler Error Index](https://doc.rust-lang.org/error-index.html)
- [The Rust Programming Language](https://doc.rust-jp.rs/book/second-edition/)
- [Rust初心者向けハンズオン](https://chikoski.github.io/rust-handson/)
- [Cargo : packages for Rust](https://crates.io/)
