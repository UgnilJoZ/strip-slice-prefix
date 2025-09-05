[![Build](https://github.com/UgnilJoZ/strip-slice-prefix/actions/workflows/rust.yaml/badge.svg)](https://github.com/UgnilJoZ/strip-slice-prefix/actions/workflows/rust.yaml)
[![Crates.io](https://img.shields.io/crates/v/strip-slice-prefix.svg)](https://crates.io/crates/strip-slice-prefix)
[![Documentation](https://docs.rs/strip-slice-prefix/badge.svg)](https://docs.rs/strip-slice-prefix/)
[![dependency status](https://deps.rs/repo/github/UgnilJoZ/strip-slice-prefix/status.svg)](https://deps.rs/repo/github/UgnilJoZ/strip-slice-prefix)

# strip-slice-prefix

This Rust crate provides a function to strip a prefix from a slice, similar to [`String::strip_prefix`](https://doc.rust-lang.org/std/string/struct.String.html#method.strip_prefix).

See the [`documentation`](https://docs.rs/strip-slice-prefix/) or this example:

```rs
use strip_slice_prefix::strip_prefix;

let greetings = b"Hello world";
let addressee = strip_prefix(greetings, b"Hello ").unwrap();
assert_eq!(addressee, b"world");
```