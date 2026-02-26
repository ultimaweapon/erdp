//! Small crate with zero dependencies to help you display an error.
#![no_std]

pub use self::wrapper::*;

use core::error::Error;
use core::fmt::Formatter;

extern crate alloc;

mod wrapper;

/// Provides a method to get a [Display].
///
/// This trait is automatically implemented for any type that implement [core::error::Error].
pub trait ErrorDisplay {
    /// Returns a [Display] to display current error and its nested errors.
    fn display(&self) -> Display<'_>;
}

impl<T: Error> ErrorDisplay for T {
    #[inline(always)]
    fn display(&self) -> Display<'_> {
        Display(self)
    }
}

impl ErrorDisplay for dyn Error {
    #[inline(always)]
    fn display(&self) -> Display<'_> {
        Display(self)
    }
}

impl ErrorDisplay for dyn Error + Send + Sync {
    #[inline(always)]
    fn display(&self) -> Display<'_> {
        Display(self)
    }
}

/// Implementation of [core::fmt::Display] to display an error and its nested errors.
pub struct Display<'a>(&'a dyn Error);

impl<'a> core::fmt::Display for Display<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        // Write top-level error.
        core::fmt::Display::fmt(self.0, f)?;

        // Write nested errors.
        let mut next = self.0.source();

        while let Some(e) = next {
            f.write_str(" -> ")?;
            core::fmt::Display::fmt(e, f)?;
            next = e.source();
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::prelude::rust_2024::*;
    use thiserror::Error;

    extern crate std;

    #[test]
    fn single() {
        let e = TestError::Single;

        assert_eq!(e.display().to_string(), "an error without nested errors");
    }

    #[test]
    fn nested() {
        let e = std::io::Error::from(std::io::ErrorKind::NotFound);
        let e = TestError::Nested(e);

        assert_eq!(e.display().to_string(), "nested error -> entity not found");
    }

    #[test]
    fn trait_object() {
        let e: Box<dyn Error> = Box::new(TestError::Single);

        assert_eq!(e.display().to_string(), "an error without nested errors");
    }

    #[derive(Debug, Error)]
    enum TestError {
        #[error("an error without nested errors")]
        Single,

        #[error("nested error")]
        Nested(#[source] std::io::Error),
    }
}
