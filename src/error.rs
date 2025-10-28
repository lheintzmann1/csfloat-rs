use thiserror::Error;

/// Result type alias for CSFloat API operations
pub type Result<T> = std::result::Result<T, Error>;

/// Error types for CSFloat API client
#[derive(Error, Debug)]
pub enum Error {
    #[error("Unauthorized -- Your API key is wrong.")]
    Unauthorized,

    #[error("Forbidden -- The requested resource is hidden for administrators only.")]
    Forbidden,

    #[error("Not Found -- The specified resource could not be found.")]
    NotFound,

    #[error("Method Not Allowed -- You tried to access a resource with an invalid method.")]
    MethodNotAllowed,

    #[error("Not Acceptable -- You requested a format that isn't json.")]
    NotAcceptable,

    #[error("Gone -- The requested resource has been removed from our servers.")]
    Gone,

    #[error("I'm a teapot.")]
    Teapot,

    #[error("Too Many Requests -- You're requesting too many resources! Slow down!")]
    TooManyRequests,

    #[error("Internal Server Error -- We had a problem with our server. Try again later.")]
    InternalServerError,

    #[error("Service Unavailable -- We're temporarily offline for maintenance. Please try again later.")]
    ServiceUnavailable,

    #[error("HTTP Error: {status} - {body}")]
    HttpError { status: u16, body: String },

    #[error("Invalid proxy URL format: {0}")]
    InvalidProxy(String),

    #[error("Request error: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("JSON parsing error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("URL parsing error: {0}")]
    UrlError(#[from] url::ParseError),

    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    #[error("Expected JSON response but got {0}")]
    UnexpectedContentType(String),
}

impl Error {
    /// Create an error from HTTP status code
    pub fn from_status(status: u16, body: String) -> Self {
        match status {
            401 => Error::Unauthorized,
            403 => Error::Forbidden,
            404 => Error::NotFound,
            405 => Error::MethodNotAllowed,
            406 => Error::NotAcceptable,
            410 => Error::Gone,
            418 => Error::Teapot,
            429 => Error::TooManyRequests,
            500 => Error::InternalServerError,
            503 => Error::ServiceUnavailable,
            _ => Error::HttpError { status, body },
        }
    }
}
