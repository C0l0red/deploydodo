use std::collections::HashMap;

use crate::impl_deref;
use axum::http::header::AUTHORIZATION;
use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};

const QUERY_TOKEN_KEY: &str = "token";

#[derive(Clone)]
pub struct BearerToken(String);

impl_deref!(BearerToken, String);

pub async fn bearer_auth(
    req: axum::http::Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let mut req = req;
    if let Some(token) = get_bearer_token_from_request(&req) {
        req.extensions_mut().insert(BearerToken(token));
    }
    Ok(next.run(req).await)
}

fn get_bearer_token_from_request(req: &Request<Body>) -> Option<String> {
    req.headers()
        .get(AUTHORIZATION)
        .and_then(|h| {
            h.to_str()
                .ok()
                .take_if(|s| !s.is_empty() && *s != "null")
                .map(ToString::to_string)
        })
        .or_else(|| get_bearer_token_from_query_params(req))
}

fn get_bearer_token_from_query_params(req: &Request<Body>) -> Option<String> {
    req.uri()
        .query()
        .and_then(|q| serde_urlencoded::from_str::<HashMap<String, String>>(q).ok())
        .and_then(|map| map.get(QUERY_TOKEN_KEY).map(ToString::to_string))
}
