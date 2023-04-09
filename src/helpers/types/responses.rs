use axum::http::StatusCode;
use axum::{
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
#[derive(Serialize, Deserialize)]
pub struct ResponseBuilder<T> {
    code: u16,
    message: Option<&'static str>,
    success: bool,
    content: Option<T>,
    error_code: Option<&'static str>,
}

impl<T: Serialize> ResponseBuilder<T> {
    pub fn success(content: Option<T>, message: Option<&'static str>, code: Option<u16>) -> Self {
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
        error_code: &'static str,
        content: Option<T>,
        message: Option<&'static str>,
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

    pub fn validation_error(content: Option<T>, message: Option<&'static str>) -> Self {
        let message = match message {
            Some(message) => message,
            None => "Validation error",
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


impl ResponseBuilder<Value>{
    pub fn paginated_response<T: Serialize>(content: &(Vec<T>, u64)) -> Self {

        let content = json!(
            {
                "data": content.0,
                "total": content.1
            }
        );


        Self {
            code: 200,
            message: None,
            success: true,
            content: Some(content),
            error_code: None,
        }
    }
}


impl<T: Serialize> IntoResponse for ResponseBuilder<T> {
    fn into_response(self) -> Response {
        let content = match self.error_code {
            Some(_) => json!(
                {
                    "message": self.message,
                    "success": self.success,
                    "content": self.content,
                    "error_code": self.error_code
                }
            ),
            None => json!(
                {
                    "message": self.message,
                    "success": self.success,
                    "content": self.content
                }
            ),
        };

        let code = StatusCode::from_u16(self.code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

        (code, Json(content)).into_response()
    }
}

pub type HandlerResponse = Result<Response, Response>;
