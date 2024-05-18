use std::error::Error;
use std::fmt::Formatter;

/// Provides a method to get a [`Display`].
///
/// This trait is automatically implemented for any type that implement [`std::error::Error`].
pub trait ErrorDisplay {
    /// Returns a [`Display`] to display the current error and its nested errors.
    fn display(&self) -> Display;
}

impl<T: Error> ErrorDisplay for T {
    fn display(&self) -> Display {
        Display(self)
    }
}

impl ErrorDisplay for dyn Error {
    fn display(&self) -> Display {
        Display(self)
    }
}

/// Implementation of [`std::fmt::Display`] for display an error and its nested errors.
pub struct Display<'a>(&'a dyn Error);

impl<'a> std::fmt::Display for Display<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // Write top-level error.
        std::fmt::Display::fmt(self.0, f)?;

        // Write nested errors.
        let mut next = self.0.source();

        while let Some(e) = next {
            f.write_str(" -> ")?;
            std::fmt::Display::fmt(e, f)?;
            next = e.source();
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use thiserror::Error;

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
