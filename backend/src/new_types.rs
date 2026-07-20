use std::fmt::Display;
use std::num::NonZeroU16;
use url::Host;
use utoipa::ToSchema;
use crate::error::AppError;
use crate::{impl_deref, impl_deserialize_via_try_new, newtype};

newtype! {
    #[derive(ToSchema, Debug)]
    pub struct PlainPassword(String);
}

impl PlainPassword {
    fn try_new(value: impl Into<String>) -> Result<Self, AppError> {
        let value = value.into().trim().to_string();

        if value.len() < 8 {
            return Err(AppError::Validation("must be at least 8 characters".into()));
        }

        Ok(Self(value))
    }
}


newtype! {
    #[derive(ToSchema, sqlx::Type, Debug)]
    #[sqlx(transparent)]
    pub struct NonEmptyString(String);
}

impl NonEmptyString {
    pub fn try_new(value: impl Into<String>) -> Result<Self, AppError> {
        let value = value.into().trim().to_owned();

        if value.is_empty() {
            return Err(AppError::Validation("must not be empty".into()));
        }

        Ok(Self(value))
    }
}

impl Display for NonEmptyString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

newtype! {
    #[derive(Debug, ToSchema)]
    pub struct ServerPort(u16);
}

impl ServerPort {
    pub fn try_new(value: impl Into<u16>) -> Result<Self, AppError> {
        NonZeroU16::new(value.into())
            .map(|value| Self(value.get()))
            .ok_or_else(|| AppError::Validation("must be between 1 and 65535".into()))
    }
}

newtype! {
    /// An abstraction of a hostname, with validation
   #[derive(Debug)]
    pub struct Hostname(String);
}

impl Hostname {
    // FIXME: This validation is not working in the way I expected. Need to find another way
    pub fn try_new(value: impl Into<String>) -> Result<Self, AppError> {
        let value = value.into().trim().to_owned();

        Host::parse(&value)
            .map_err(|_| AppError::Validation("must be a valid host".into()))?;

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

newtype! {
    deserialize_as(String)
    deref_as(String)
    
    #[derive(ToSchema, sqlx::Type)]
    #[sqlx(transparent)]
    pub struct SshPublicKey(NonEmptyString);
}

impl From<NonEmptyString> for String {
    fn from(value: NonEmptyString) -> Self {
        value.into()
    }
}

impl SshPublicKey {
    pub fn try_new(value: impl Into<String>) -> Result<Self, AppError> {
        let value = NonEmptyString::try_new(value)?;

        Ok(Self(value))
    }
}

newtype! {
    deserialize_as(String)
    deref_as(String)

    #[derive(ToSchema, sqlx::Type)]
    #[sqlx(transparent)]
    pub struct SshPrivateKey(NonEmptyString);
}


impl SshPrivateKey {
    pub fn try_new(value: impl Into<String>) -> Result<Self, AppError> {
        let value = NonEmptyString::try_new(value)?;
        if !value.starts_with("-----BEGIN")
        {
            return Err(AppError::Validation("must be a valid SSH private key".into()));
        }

        Ok(Self(value))
    }
}

#[cfg(test)]
mod tests {
    use crate::new_types::{Hostname, NonEmptyString, PlainPassword, ServerPort};

    #[test]
    fn non_empty_string_rejects_blank_values() {
        let err = serde_json::from_str::<NonEmptyString>(r#""   ""#)
            .expect_err("whitespace-only strings must be rejected");

        assert_eq!(err.to_string(), "must not be empty");
    }

    #[test]
    fn non_empty_string_accepts_text() {
        let value = serde_json::from_str::<NonEmptyString>(r#""Ada""#).unwrap();

        assert_eq!(value.0, "Ada");
    }

    #[test]
    fn password_rejects_short_values() {
        let err = serde_json::from_str::<PlainPassword>(r#""short""#)
            .expect_err("passwords shorter than 8 chars must be rejected");

        assert_eq!(
            err.to_string(),
            "must be at least 8 characters"
        );
    }

    #[test]
    fn password_accepts_eight_or_more_characters() {
        let value = serde_json::from_str::<PlainPassword>(r#""password""#).unwrap();

        assert_eq!(value.0, "password");
    }

    #[test]
    fn server_port_rejects_zero() {
        let err = serde_json::from_str::<ServerPort>("0")
            .expect_err("port 0 must be rejected");

        assert_eq!(err.to_string(), "must be between 1 and 65535");
    }

    #[test]
    fn server_port_accepts_valid_ports() {
        let value = serde_json::from_str::<ServerPort>("8080").unwrap();

        assert_eq!(value.0, 8080);
    }

    #[test]
    fn host_rejects_empty_values() {
        // These tests prove that this package does not work as expected. Need to find a different solution
        Hostname::try_new("google.com").expect("Could not parse URL");
        Hostname::try_new("https://google.com").expect_err("Full URL must be rejected");
        Hostname::try_new("").expect_err("Empty host must be rejected");
        Hostname::try_new("abc").expect("Could not parse IPv4");
    }
}