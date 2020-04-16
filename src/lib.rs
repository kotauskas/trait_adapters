//! # Version notice
//! **This is an early version of the crate, released due to high demand in a different crate. Features listed below might not exist yet: additions will be noted in the changelog.**
//!
//! Provides newtype adapters for modifying interactions with traits.
//!
//! Examples include:
//! - ~~Removing all traits from a type and leaving only one (customized thanks to macros)~~
//! - ~~Implementing a trait by using another trait with an equivalent signature (also customized thanks to macros; a good use case is the variety of formatting traits in `std::fmt`)~~
//! - Containers which are alternatives to equivalent standard library containers but use slightly different traits or alter their meaning

#![no_std]
extern crate alloc;

pub mod bow;
pub use bow::*;