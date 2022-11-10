# Safe Linked Lists for Rust

Linked Lists are notoriously difficult to implement in Rust without using [`unsafe`](https://doc.rust-lang.org/std/keyword.unsafe.html). Even [the standard library implementation](https://github.com/rust-lang/rust/blob/master/library/alloc/src/collections/linked_list.rs) uses unsafe code.

This is my own implementation. It does not use `unsafe` anywhere.
