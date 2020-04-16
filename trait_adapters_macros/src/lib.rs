//! Macros for the `trait_adapters` crate.
//!
//! Currently there are none, this is obviously expected to change in the future.
// Currently, there's no way of reexporting macros using pub use, so the only way to organize them is to export wrappers instead of the actual macro code.
extern crate proc_macro;
/*use proc_macro::TokenStream;

mod isolator;
use isolator::trait_isolator_impl;

/// Generates a trait isolator for the specified trait.
#[proc_macro]
pub fn trait_isolator(input: TokenStream) -> TokenStream {
    trait_isolator_impl(input)
}*/