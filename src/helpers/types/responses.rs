use axum::http::StatusCode;
use axum::{
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize)]
pub struct ResponseBuilder<T: Serialize> {
    code: u16,
    message: Option<String>,
    success: bool,
    content: Option<T>,
}

impl<T: Serialize> ResponseBuilder<T> {
    pub fn success(content: Option<T>, message: Option<String>, code: Option<u16>) -> Self {
        let code = match code {
            Some(code) => {
                if code < 200 || code > 299 {
                    200
                } else {
                    code
                }
            }
            None => 200,
        };

        Self {
            code,
            message,
            success: true,
            content,
        }
    }

    pub fn error(content: Option<T>, message: Option<String>, code: Option<u16>) -> Self {
        
        let code = match code {
            Some(code) => {
                if code < 400 || code > 599 {
                    500
                } else {
                    code
                }
            }
            None => 400,
        };

        Self {
            code,
            message,
            success: false,
            content,
        }
    }

    pub fn validation_error(content: Option<T>, message: Option<String>) -> Self {

        let message = match message {
            Some(message) => message,
            None => String::from("Validation error"),
        };

        Self {
            code: 422,
            message: Some(message),
            success: false,
            content,
        }
    }
}

impl<T: Serialize> IntoResponse for ResponseBuilder<T> {
    fn into_response(self) -> Response {
        let content = json!({
            "message": self.message,
            "success": self.success,
            "content": self.content
        });

        let code = StatusCode::from_u16(self.code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

        (code, Json(content)).into_response()
    }
}

pub type HandlerResponse = Result<Response, Response>;
