//! A clone-on-write smart pointer with heap allocation.

use alloc::{boxed::Box, borrow::Borrow};
use core::fmt;

/// A clone-on-write smart pointer with heap allocation.
///
/// This is mainly an alternative to `Cow<'a, T>`, but the owned state for the value is `Box<T>` instead of `T`. Primarily useful for recursive types (imagine the cons lists from the Rust Book, but optionally `static` without any heap allocation whatsoever), or for avoiding stack overflows.
///
/// The reason for this being located in this crate and not any other one is because it is indeed a trait adapter and not just a smart pointer: the standard-library `Cow` uses `ToOwned` for cloning, while `Bow` uses `Clone`, i.e. applies `ToOwned` semantics (cloning into a smart pointer) to `Clone` types, which automatically implement `ToOwned` by returning the type itself rather than a `Box` around it.
pub enum Bow<'a, T: Clone> {
    /// Borrowed state. The value is not owned and has to be cloned if a mutable reference is required.
    Borrowed(&'a T),
    /// Owned state. Full access to the value is provided.
    Owned(Box<T>)
}
impl<'a, T: Clone> Bow<'a, T> {
    /// Returns `true` if in borrowed state (i.e. obtaining a mutable reference requires cloning), `false` otherwise.
    #[inline]
    pub fn is_borrowed(&self) -> bool {
        match *self {
            Self::Borrowed(_) => true,
            Self::Owned(_) => false
        }
    }
    /// Returns `true` if in owned state (i.e. obtaining a mutable reference is free), `false` otherwise.
    #[inline]
    pub fn is_owned(&self) -> bool {
        match *self {
            Self::Borrowed(_) => false,
            Self::Owned(_) => true
        }
    }
    /// Retrieves a mutable reference to the inner value, cloning it if it's not owned.
    ///
    /// Use `is_owned` to check whether this is expensive or not.
    #[inline]
    pub fn to_mut(&mut self) -> &mut T {
        match self {
            Self::Borrowed(b) => {
                *self = Self::Owned(Box::from(b.clone()));
                self.to_mut() // At this point, the previous statement ensures
                              // that the Bow is owned. This means that calling
                              // this exact method takes us to the Owned branch,
                              // which breaks the recursion and returns the owned
                              // value.
            }
            Self::Owned(ref mut b) => {b}
        }
    }
    /// Retrieves the stored value, cloning it if it's not owned.
    #[inline]
    #[must_use = "cloning the value only to discard it afterwards is pointless since cloning is not expected to have side effects"]
    pub fn into_owned(self) -> T {
        match self {
            Self::Borrowed(b) => {b.clone()}
            Self::Owned(b) => {*b}
        }
    }
    /// Retrieves the stored value without dereferencing & deallocating the inner `Box`.
    #[inline]
    pub fn into_box(self) -> Box<T> {
        match self {
            Self::Borrowed(b) => {Box::from(b.clone())},
            Self::Owned(b) => {b}
        }
    }
}
impl<T: Clone> core::ops::Deref for Bow<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        self.as_ref()
    }
}
impl<T: Clone> AsRef<T> for Bow<'_, T> {
    #[inline]
    fn as_ref(&self) -> &T {
        match self {
            Self::Borrowed(b) => b,
            Self::Owned(b) => &b
        }
    }
}
impl<T: Clone> Borrow<T> for Bow<'_, T> {
    #[inline]
    fn borrow(&self) -> &T {
        self.as_ref()
    }
}

impl<'a, T: Clone> From<&'a T> for Bow<'a, T> {
    #[inline(always)]
    fn from(op: &'a T) -> Self {
        Self::Borrowed(op)
    }
}
impl<'a, T: Clone> From<Box<T>> for Bow<'a, T> {
    #[inline(always)]
    fn from(op: Box<T>) -> Self {
        Self::Owned(op)
    }
}
impl<T: Clone> From<T> for Bow<'_, T> {
    #[inline(always)]
    fn from(op: T) -> Self {
        Self::Owned(Box::from(op))
    }
}

impl<T: Clone> Clone for Bow<'_, T> {
    #[inline(always)]
    fn clone(&self) -> Self {
        Self::Owned(
            Box::from(
                self.as_ref().clone()
            )
        )
    }
}
impl<T: Clone + fmt::Debug> fmt::Debug for Bow<'_, T> {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Borrowed(b) => fmt::Debug::fmt(b, f),
            Self::Owned(b) => fmt::Debug::fmt(b, f)
        }
    }
}
impl<T: Clone + fmt::Display> fmt::Display for Bow<'_, T> {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Borrowed(b) => fmt::Display::fmt(b, f),
            Self::Owned(b) => fmt::Display::fmt(b, f)
        }
    }
}
impl<T: Clone + Default> Default for Bow<'_, T> {
    #[inline(always)]
    fn default() -> Self {
        Self::Owned(Box::from(T::default()))
    }
}
