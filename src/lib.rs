//! # Clock Tower
//!
//! `clock_tower` is a simple, thread-safe utility for measuring the execution time
//! of code blocks and expressions.
//!
//! ## Usage
//!
//! Use the `measure!` macro to wrap code you want to time.
//! Retrieve results using the `time_registry` module.

pub mod time_registry;
mod macros;