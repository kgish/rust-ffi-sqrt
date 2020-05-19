# Rust FFI Sqrt

An example of using FFI in rust in order to call the external C function sqrt.

## Installation

```
$ git clone https://github.com/kgish/rust-ffi-sqrt.git
$ cd rust-ffi-sqrt
$ cargo run
```

Hopefully, you should see something like this:

```
$ cargo run
   Compiling rust-ffi-sqrt v0.1.0 (/home/kiffin/projects/rust/ffi-sqrt)
    Finished dev [unoptimized + debuginfo] target(s) in 0.15s
     Running `target/debug/rust-ffi-sqrt`
The square root of 2 is 1.4142135623730951
```

## References

* [The Rustonomicon](https://doc.rust-lang.org/nomicon/ffi.html)
* [The Rust Book](https://doc.rust-lang.org/1.9.0/book/ffi.html)
