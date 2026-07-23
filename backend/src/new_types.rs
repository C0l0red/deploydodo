use crate::error::{AppError, AppResult};
use crate::{impl_deref, impl_deserialize_via_try_new, impl_sqlx_type_via, newtype};
use argon2::password_hash::{Encoding, SaltString};
use argon2::{Argon2, PasswordHasher, PasswordVerifier};
use rand_core::OsRng;
use serde::Serialize;
use std::fmt::{Debug, Display};
use std::num::NonZeroU16;
use std::str::FromStr;
use utoipa::ToSchema;

newtype! {
    pub struct PlainPassword(String);
}

impl PlainPassword {
    fn try_new(value: impl Into<String>) -> Result<Self, AppError> {
        let value = value.into().to_string();

        if value.len() < 8 {
            return Err(AppError::Validation("must be at least 8 characters".into()));
        }

        Ok(Self(value))
    }
}

impl Debug for PlainPassword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("PlainPassword").field(&"***").finish()
    }
}

#[cfg(test)]
impl From<&str> for PlainPassword {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

newtype! {
    pub struct HashedPassword(String);
}

impl HashedPassword {
    pub fn try_new(value: impl Into<String>) -> Result<Self, AppError> {
        let value = value.into().to_string();
        argon2::PasswordHash::parse(value.as_str(), Encoding::default())
            .map_err(|_| AppError::CouldNotParse("Password hash".to_string()))?;

        Ok(Self(value))
    }

    pub fn hash(password: &PlainPassword) -> AppResult<Self> {
        let salt = SaltString::generate(&mut OsRng);
        Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .map_err(|_| AppError::PasswordHash)
            .map(Into::into)
    }

    pub fn verify(&self, plain_password: &PlainPassword) -> AppResult<()> {
        let parsed_hash = argon2::PasswordHash::new(self)
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;
        Argon2::default()
            .verify_password(plain_password.as_bytes(), &parsed_hash)
            .map_err(|_| AppError::InvalidCredentials)
    }
}

impl Debug for HashedPassword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("HashedPassword").field(&"***").finish()
    }
}

impl From<argon2::PasswordHash<'_>> for HashedPassword {
    fn from(value: argon2::PasswordHash) -> Self {
        Self(value.to_string())
    }
}

newtype! {
    #[derive(Debug)]
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
    no_extra_derives
    #[derive(Debug, Copy, Clone, Serialize, ToSchema)]
    pub struct ServerPort(u16);
}

impl_sqlx_type_via!(ServerPort => i32);

impl FromStr for ServerPort {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        u16::from_str(s)
            .map_err(|err| AppError::CouldNotParse(err.to_string()))
            .map(Self)
    }
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

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

newtype! {
    deref_as(String)
    deserialize_as(String)

    pub struct SshPublicKey(NonEmptyString);
}

impl From<NonEmptyString> for String {
    fn from(value: NonEmptyString) -> Self {
        value.0
    }
}

impl SshPublicKey {
    pub fn try_new(value: impl Into<String>) -> Result<Self, AppError> {
        let value = NonEmptyString::try_new(value)?;

        Ok(Self(value))
    }
}

newtype! {
    deref_as(String)
    deserialize_as(String)

    pub struct SshPrivateKey(NonEmptyString);
}

impl SshPrivateKey {
    pub fn try_new(value: impl Into<String>) -> Result<Self, AppError> {
        let value = NonEmptyString::try_new(value)?;
        if !value.starts_with("-----BEGIN") {
            return Err(AppError::Validation(
                "must be a valid SSH private key".into(),
            ));
        }

        Ok(Self(value))
    }
}

#[cfg(test)]
mod tests {
    use crate::new_types::{
        HashedPassword, NonEmptyString, PlainPassword, ServerPort, SshPrivateKey, SshPublicKey,
    };

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
        let res = serde_json::from_str::<PlainPassword>(r#""short""#);
        assert!(
            res.is_err(),
            "passwords shorter than 8 chars must be rejected"
        );
        let err = res.err().unwrap();
        assert_eq!(err.to_string(), "must be at least 8 characters");
    }

    #[test]
    fn password_accepts_eight_or_more_characters() {
        let value = serde_json::from_str::<PlainPassword>(r#""password""#).unwrap();

        assert_eq!(value.0, "password");
    }

    #[test]
    fn server_port_rejects_zero() {
        let err = serde_json::from_str::<ServerPort>("0").expect_err("port 0 must be rejected");

        assert_eq!(err.to_string(), "must be between 1 and 65535");
    }

    #[test]
    fn server_port_accepts_valid_ports() {
        let value = serde_json::from_str::<ServerPort>("8080").unwrap();

        assert_eq!(value.0, 8080);
    }

    // Additional tests for other new types
    #[test]
    fn hashed_password_rejects_invalid_hash() {
        let res = HashedPassword::try_new("not-a-valid-hash");
        assert!(res.is_err(), "invalid hash must be rejected");
        assert_eq!(
            res.err().unwrap().to_string(),
            "could not parse: Password hash"
        );
    }

    #[test]
    fn hashed_password_accepts_valid_argon2_hash() {
        // This is a minimal, syntactically valid Argon2id hash string
        let hash = "$argon2id$v=19$m=65536,t=3,p=4$c29tZXNhbHQ$c29tZWNoZWNrc3Vt";
        HashedPassword::try_new(hash).expect("valid argon2 hash should be accepted");
    }

    #[test]
    fn ssh_public_key_rejects_empty() {
        let res = serde_json::from_str::<SshPublicKey>(r#"""#);
        assert!(res.is_err(), "empty public key must be rejected");
    }

    #[test]
    fn ssh_public_key_accepts_non_empty() {
        let key =
            serde_json::from_str::<SshPublicKey>(r#""ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAABAQCs""#)
                .unwrap();
        assert!(!key.as_str().is_empty());
    }

    #[test]
    fn ssh_private_key_rejects_invalid() {
        let res = serde_json::from_str::<SshPrivateKey>(r#""not-a-key""#);
        assert!(res.is_err(), "invalid private key must be rejected");
    }

    #[test]
    fn ssh_private_key_accepts_pem_like() {
        let pem = r#""-----BEGIN OPENSSH PRIVATE KEY-----\n-----END OPENSSH PRIVATE KEY-----""#;
        serde_json::from_str::<SshPrivateKey>(pem).expect("valid-looking key should be accepted");
    }
}
