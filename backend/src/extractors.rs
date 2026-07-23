use crate::{
    dependencies::Dependencies, error::AppError, middleware::BearerToken, services::types::User,
};
use axum::extract::rejection::JsonRejection;
use axum::extract::{FromRequest, Request};
use axum::response::{IntoResponse, Response};
use axum::{extract::FromRequestParts, http::request::Parts, Json};
use axum_test::expect_json::__private::serde_trampoline::de::DeserializeOwned;
use serde_json::json;

pub struct Auth(pub User);

impl FromRequestParts<Dependencies> for Auth {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        deps: &Dependencies,
    ) -> Result<Self, Self::Rejection> {
        if let Some(bearer_token) = parts.extensions.get::<BearerToken>() {
            let user = deps.session_service.get_session_user(bearer_token).await?;

            if let Some(user) = user {
                return Ok(Auth(user));
            }
        }
        Err(AppError::Unauthorized)
    }
}

pub struct MaybeAuth(pub Option<User>);

impl FromRequestParts<Dependencies> for MaybeAuth {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        deps: &Dependencies,
    ) -> Result<Self, Self::Rejection> {
        match Auth::from_request_parts(parts, deps)
            .await
            .map(|auth| auth.0)
        {
            Ok(user) => Ok(MaybeAuth(Some(user))),
            Err(AppError::Unauthorized) => Ok(MaybeAuth(None)),
            Err(err) => Err(err),
        }
    }
}

/// This is an extractor for JSON requests, to clean up the response bodies to when there's a 422 error
pub struct RequestJson<T>(pub T);

impl<S, T> FromRequest<S> for RequestJson<T>
where
    S: Send + Sync,
    T: DeserializeOwned,
{
    type Rejection = Response;

    /// This extractor function exists specifically to tackle the messy plain text error body returned
    /// by Axum and serde's JSON deserialization. It cleans out parts that shouldn't be returned to
    /// the client and wraps errors in a JSON body.
    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        Json::<T>::from_request(req, state)
            .await
            .map(|Json(value)| Self(value))
            .map_err(|err| match err {
                JsonRejection::JsonDataError(err) => {
                    let body_text = err.body_text();

                    // Chops off parts of the response that give unnecessary information to the client
                    let message = body_text
                        .strip_prefix("Failed to deserialize the JSON body into the target type: ")
                        .map_or(body_text.as_str(), |message| message)
                        .split(" at line ")
                        .next()
                        .unwrap_or(body_text.as_str());
                    let json = json!({"message": message});

                    (err.status(), Json(json)).into_response()
                }
                err => (err.status(), Json(json!({"message": err.body_text()}))).into_response(),
            })
    }
}
