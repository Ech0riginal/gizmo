//! These are happy path/sanity tests - not
pub(crate) mod diff;
pub(self) mod macros;

#[cfg(feature = "happy_paths")]
mod v2;

#[cfg(feature = "happy_paths")]
mod v3;
