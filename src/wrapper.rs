use alloc::borrow::Cow;
use core::error::Error;
use core::fmt::{Display, Formatter};

/// Helper function to construct [ErrorWrapper].
///
/// `m` is a string to describe current error. `e` is a cause of current error.
#[inline(always)]
pub fn wrap<E>(m: impl Into<Cow<'static, str>>, e: E) -> ErrorWrapper<E>
where
    E: Error + 'static,
{
    ErrorWrapper {
        message: m.into(),
        error: e,
    }
}

/// Encapsulates error message and its inner error.
#[derive(Debug)]
pub struct ErrorWrapper<E> {
    /// Message for current error.
    pub message: Cow<'static, str>,
    /// Cause of current error.
    pub error: E,
}

impl<E> Error for ErrorWrapper<E>
where
    E: Error + 'static,
{
    #[inline(always)]
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.error)
    }
}

impl<E> Display for ErrorWrapper<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        Display::fmt(&self.message, f)
    }
}
