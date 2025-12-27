use std::fmt;
use std::fmt::Display;

/// Represents a HTTP status identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum Status {
    Ok = 200,
    NotFound = 404,
    InternalServerError = 500,
}

impl Status {
    /// Returns the numerical HTTP status code.
    pub fn code(&self) -> u16 {
        *self as u16
    }

    /// Returns a standard text message related to the status code.
    pub fn message(&self) -> &'static str {
        match self {
            Self::Ok => "OK",
            Self::NotFound => "Not Found",
            Self::InternalServerError => "Internal Server Error",
        }
    }
}

impl Display for Status {
    /// Formats HTTP status as a string containing the code and message.
    ///
    /// Resulting format: `{code} {message}`.
    ///
    /// # Examples
    ///
    /// ```
    /// let status = Status::NotFound;
    /// assert_eq!(format!("{}", status), "404 Not Found");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.code(), self.message())?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_code() {
        let status = Status::Ok;

        assert_eq!(200, status.code());
    }

    #[test]
    fn test_message() {
        let status = Status::InternalServerError;

        assert_eq!("Internal Server Error", status.message());
    }

    #[test]
    fn test_format() {
        let status = Status::NotFound;

        assert_eq!(format!("{}", status), "404 Not Found");
    }
}
