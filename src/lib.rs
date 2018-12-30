//! # hello_world Crate
//!
//! `hello_world` is a collection of `The Rust Programming Language - second edition`'s scripts.

// set original modules in this file

/// main module is `my_module`

pub mod my_module;

/// main test module is `my_test`

pub mod my_test;

// using crates

#[macro_use]
extern crate failure;
extern crate regex;
