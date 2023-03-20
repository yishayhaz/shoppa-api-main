use crate::helpers::types::ResponseBuilder;
use axum::{
    async_trait,
    extract::{ConnectInfo, FromRequestParts},
    http::{request::Parts, Extensions},
    response::{IntoResponse, Response},
};
use std::{
    marker::Sync,
    net::{IpAddr, SocketAddr},
};

pub struct ClientIpAddress(pub IpAddr);

// source https://github.com/imbolc/axum-client-ip/
#[async_trait]
impl<S> FromRequestParts<S> for ClientIpAddress
where
    S: Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts
            .headers
            .get("X-Real-Ip")
            .and_then(|hv| hv.to_str().ok())
            .and_then(|s| s.parse::<IpAddr>().ok())
            .or_else(|| maybe_connect_info(&parts.extensions))
            .map(Self)
            .ok_or(
                ResponseBuilder::<u16>::error(
                    // TODO add eror code here
                    "",
                    None,
                    Some("faild to get client IpAddress"),
                    Some(500),
                )
                .into_response(),
            )
    }
}

fn maybe_connect_info(extensions: &Extensions) -> Option<IpAddr> {
    extensions
        .get::<ConnectInfo<SocketAddr>>()
        .map(|ConnectInfo(addr)| addr.ip())
}
