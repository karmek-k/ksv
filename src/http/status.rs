/// Status is an enum holding HTTP status identifiers.
/// 
/// In order to get a status code and message, use the `tuple` method.
#[allow(dead_code)]
pub enum Status {
    // Success
    Ok,

    // Client-side Failure
    NotFound,

    // Server-side Failure
    InternalServerError,
}

impl Status {
    /// Returns the status code and message of a `Status` enum instance.
    pub fn tuple(&self) -> (i32, &'static str) {
        // less typing
        type S = Status;

        match self {
            // Success
            S::Ok => (200, "OK"),

            // Client-side Failure
            S::NotFound => (404, "Not Found"),

            // Server-side Failure
            S::InternalServerError => (500, "Internal Server Error"),
        }
    }
}