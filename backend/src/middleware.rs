use std::collections::HashMap;

use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};

#[derive(Clone)]
pub struct BearerToken(String);

impl BearerToken {
    pub fn token(&self) -> &str {
        self.0.as_ref()
    }
}

pub async fn bearer_auth(
    req: axum::http::Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let mut req = req;
    if let Some(token) = bearer_token(&req) {
        req.extensions_mut().insert(BearerToken(token));
    }
    Ok(next.run(req).await)
}

fn bearer_token(req: &Request<Body>) -> Option<String> {
    req.headers()
        .get("Authorization")
        .and_then(|h| {
            h.to_str()
                .ok()
                .take_if(|s| !s.is_empty() && *s != "null")
                .map(ToString::to_string)
        })
        .or_else(|| query_param(req, "token"))
}

fn query_param(req: &Request<Body>, key: &str) -> Option<String> {
    req.uri()
        .query()
        .and_then(|q| serde_urlencoded::from_str::<HashMap<String, String>>(q).ok())
        .and_then(|map| map.get(key).map(ToString::to_string))
}
