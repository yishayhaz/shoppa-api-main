use crate::helpers::env::ENV_VARS;
use http::{header, request::Parts as RequestParts, HeaderName, HeaderValue, Method};
use tower_http::cors::{AllowHeaders, AllowOrigin, CorsLayer};

pub fn get_cors_layer() -> CorsLayer {
    let methods = [
        Method::POST,
        Method::GET,
        Method::PUT,
        Method::PATCH,
        Method::DELETE,
        Method::OPTIONS,
        Method::HEAD,
    ];

    let headers = [
        header::CONTENT_TYPE,
        header::ACCEPT,
        header::ACCEPT_CHARSET,
        header::ACCEPT_ENCODING,
        header::ACCEPT_LANGUAGE,
        header::ACCEPT_RANGES,
        header::ACCESS_CONTROL_ALLOW_CREDENTIALS,
        header::ACCESS_CONTROL_ALLOW_HEADERS,
        header::ACCESS_CONTROL_ALLOW_METHODS,
        header::ACCESS_CONTROL_ALLOW_ORIGIN,
        header::ACCESS_CONTROL_EXPOSE_HEADERS,
        header::ACCESS_CONTROL_MAX_AGE,
        header::ACCESS_CONTROL_REQUEST_HEADERS,
        header::ACCESS_CONTROL_REQUEST_METHOD,
        header::AGE,
        header::ALLOW,
        header::ALT_SVC,
        header::AUTHORIZATION,
        header::CACHE_CONTROL,
        header::CACHE_STATUS,
        header::CDN_CACHE_CONTROL,
        header::CONNECTION,
        header::CONTENT_DISPOSITION,
        header::CONTENT_ENCODING,
        header::CONTENT_LANGUAGE,
        header::CONTENT_LENGTH,
        header::CONTENT_LOCATION,
        header::CONTENT_RANGE,
        header::CONTENT_SECURITY_POLICY,
        header::CONTENT_SECURITY_POLICY_REPORT_ONLY,
        header::CONTENT_TYPE,
        header::COOKIE,
        header::DNT,
        header::DATE,
        header::ETAG,
        header::EXPECT,
        header::EXPIRES,
        header::FORWARDED,
        header::FROM,
        header::HOST,
        header::IF_MATCH,
        header::IF_MODIFIED_SINCE,
        header::IF_NONE_MATCH,
        header::IF_RANGE,
        header::IF_UNMODIFIED_SINCE,
        header::LAST_MODIFIED,
        header::LINK,
        header::LOCATION,
        header::MAX_FORWARDS,
        header::ORIGIN,
        header::PRAGMA,
        header::PROXY_AUTHENTICATE,
        header::PROXY_AUTHORIZATION,
        header::PUBLIC_KEY_PINS,
        header::PUBLIC_KEY_PINS_REPORT_ONLY,
        header::RANGE,
        header::REFERER,
        header::REFERRER_POLICY,
        header::REFRESH,
        header::RETRY_AFTER,
        header::SEC_WEBSOCKET_ACCEPT,
        header::SEC_WEBSOCKET_EXTENSIONS,
        header::SEC_WEBSOCKET_KEY,
        header::SEC_WEBSOCKET_PROTOCOL,
        header::SEC_WEBSOCKET_VERSION,
        header::SERVER,
        header::SET_COOKIE,
        header::STRICT_TRANSPORT_SECURITY,
        header::TE,
        header::TRAILER,
        header::TRANSFER_ENCODING,
        header::UPGRADE,
        header::UPGRADE_INSECURE_REQUESTS,
        header::USER_AGENT,
        header::VARY,
        header::VIA,
        header::WARNING,
        header::WWW_AUTHENTICATE,
        header::X_CONTENT_TYPE_OPTIONS,
        header::X_DNS_PREFETCH_CONTROL,
        header::X_FRAME_OPTIONS,
        header::X_XSS_PROTECTION,
        HeaderName::from_static("do_connecting-ip"),
    ];

    CorsLayer::new()
        .allow_methods(methods)
        .allow_credentials(true)
        .allow_origin(AllowOrigin::predicate(
            |origin: &HeaderValue, _request_parts: &RequestParts| {
                if ENV_VARS.is_production() {
                    return origin.as_bytes().ends_with(ENV_VARS.CORS_DOMAIN.as_bytes());
                }
                true
            },
        ))
        .allow_headers(AllowHeaders::list(headers))
}
