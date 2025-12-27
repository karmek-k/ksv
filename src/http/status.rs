use std::fmt;
use std::fmt::Display;

/// Represents a HTTP status identifier.
#[allow(dead_code)]
pub enum Status {
    Ok,
    NotFound,
    InternalServerError,
}

// TODO: change this implementation
impl Status {
    /// Returns the status code and message of a `Status` enum instance.
    pub fn tuple(&self) -> (i32, &'static str) {
        type S = Status;

        match self {
            S::Ok => (200, "OK"),
            S::NotFound => (404, "Not Found"),
            S::InternalServerError => (500, "Internal Server Error"),
        }
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (status_code, status_text) = self.tuple();
        write!(f, "{} {}", status_code, status_text)?;

        Ok(())
    }
}
