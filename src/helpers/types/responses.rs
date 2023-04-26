use axum::http::StatusCode;
use axum::{
    response::{IntoResponse, Response},
    Json,
};
use bson::oid::ObjectId;
use mongodb::error::Error;
use serde::Serialize;
use serde_json::{json, Value};

#[derive(Serialize, Debug)]
pub struct ResponseBuilder<'a, T: Serialize> {
    #[serde(skip)]
    code: u16,
    message: Option<&'a str>,
    success: bool,
    // #[serde(serialize_with = "serialize_any_object_id_as_string")]
    content: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error_code: Option<&'a str>,
}

impl<'a, T: Serialize> ResponseBuilder<'a, T> {
    pub fn success(content: Option<T>, message: Option<&'a str>, code: Option<u16>) -> Self {
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
        error_code: &'a str,
        content: Option<T>,
        message: Option<&'a str>,
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

    pub fn validation_error(content: Option<T>, message: Option<&'a str>) -> Self {
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

impl ResponseBuilder<'_, Value> {
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

impl <'a> ResponseBuilder<'a, String> {
    pub fn cursor_consumpetion_error(collection: &'a str, error: Error) -> Self {
        let kind = *error.kind;

        Self {
            code: 500,
            message: Some(collection),
            success: false,
            content: Some(kind.to_string()),
            error_code: Some("consumpetion_error"),
        }
    }

    pub fn query_error(collection: &'a str, error: Error) -> Self {
        let kind = *error.kind;
        Self {
            code: 500,
            message: Some(collection),
            success: false,
            content: Some(kind.to_string()),
            error_code: Some("query_error"),
        }
    }

    pub fn not_found_error(collection: &'a str, id: &ObjectId) -> Self {
        Self {
            code: 404,
            message: Some(collection),
            success: false,
            content: Some(id.to_hex()),
            error_code: Some("not_found_error"),
        }
    }
}

impl<'a, T: Serialize> IntoResponse for ResponseBuilder<'a, T> {
    fn into_response(self) -> Response {
        let code = StatusCode::from_u16(self.code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

        (code, Json(self)).into_response()
    }
}
