use axum::response::{IntoResponse, Response};

// Main Crate Error

#[derive(Debug)]
pub enum Error {
	/// For starter, to remove as code matures.
	Generic(String),
	/// For starter, to remove as code matures.
	Static(&'static str),

}


impl IntoResponse for Error {

    fn into_response(self) -> Response {
        Response::default()
    }
}


impl Error {
    /// For starter, to remove as code matures.
    pub fn new<S: Into<String>>(msg: S) -> Self {
        Self::Generic(msg.into())
    }
}

