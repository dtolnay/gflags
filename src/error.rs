use std::fmt::{self, Display};

/// Error returned when parsing a flag value fails.
///
/// This is the error type returned by [`Value::parse`].
///
/// [`Value::parse`]: trait.Value.html
#[derive(Debug)]
pub struct Error {
    message: String,
}

/// Result of parsing a flag value.
///
/// This is the result type returned by [`Value::parse`].
///
/// [`Value::parse`]: trait.Value.html
pub type Result<T> = std::result::Result<T, Error>;

impl Error {
    pub fn new<T: Display>(message: T) -> Self {
        Error {
            message: message.to_string(),
        }
    }
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

#[doc(hidden)]
#[allow(non_snake_case)]
pub fn Error<T: Display>(message: T) -> Error {
    Error::new(message)
}
