use axum::http::StatusCode;
use axum::{
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
#[derive(Serialize, Deserialize)]
pub struct ResponseBuilder<T: Serialize> {
    #[serde(skip)]
    code: u16,
    message: Option<String>,
    success: bool,
    content: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error_code: Option<&'static str>,
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
            error_code: None,
        }
    }

    pub fn error(
        error_code: &str,
        content: Option<T>,
        message: Option<String>,
        code: Option<u16>,
    ) -> Self {
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
            error_code: Some(error_code),
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
            error_code: None,
        }
    }
}

impl<T: Serialize> IntoResponse for ResponseBuilder<T> {
    fn into_response(self) -> Response {
        let content = serde_json::to_value(self).unwrap_or_else(|_| {
            self.code = 500;

            return json!({
                "success": false,
                "message": "faild serializing body",
                "content": null
            });
        });

        let code = StatusCode::from_u16(self.code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

        (code, Json(content)).into_response()
    }
}

pub type HandlerResponse = Result<Response, Response>;
