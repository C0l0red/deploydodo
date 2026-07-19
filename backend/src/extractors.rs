use axum::{extract::FromRequestParts, http::request::Parts};

use crate::{
    dependencies::Dependencies, error::AppError, middleware::BearerToken, services::types::User,
};

pub struct Auth(pub User);

impl FromRequestParts<Dependencies> for Auth {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        deps: &Dependencies,
    ) -> Result<Self, Self::Rejection> {
        if let Some(bearer_token) = parts.extensions.get::<BearerToken>() {
            let user = deps
                .session_service
                .get_session_user(bearer_token.token())
                .await?;

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
