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
