//! Provides newtype adapters for modifying interactions with traits.
//!
//! Examples include:
//! - Removing all traits from a type and leaving only one (customized thanks to macros)
//! - Implementing a trait by using another trait with an equivalent signature (also customized thanks to macros; a good use case is the variety of formatting traits in `std::fmt`)
//! - Editing associated types of a trait implementation in order to fix infinite recursion or other issues related to compiler understanding
//!
//!   The case being referred to is mainly `std::borrow::Cow`, which stores either a borrowed value or the output of its `std::borrow::AsOwned` implementation, which is `Self` for types that are `Clone`. Changing that from `Self` to `Box<Self>` will prevent a data structure from trying to store itself without pointer indirection.

#![no_std]
extern crate alloc;

pub mod bow;
pub use bow::*;