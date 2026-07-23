/// Generates a database entity and an associated struct without the ID, to allow for easier creation of new entities
#[macro_export]
macro_rules! entity {
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident {
            $id_field_name:ident : $id_ty:ident,
            $(
                $field:ident : $ty:ty
            ),* $(,)?
        }
    ) => {
        // Invoke entity_id! macro
        entity_id! {
            $vis struct $id_ty;
        }

        $(#[$meta])*
        $vis struct $name {
            pub $id_field_name: $id_ty,
            $(
                pub $field: $ty,
            )*
        }

        paste::paste! {
            $(#[$meta])*
            $vis struct [<$name Input>] {
                $(
                    pub $field: $ty,
                )*
            }
        }
    };
}

/// Implements `Deserialize` in a way that relies on the struct implementing a `try_new` associated function
#[macro_export]
macro_rules! impl_deserialize_via_try_new {
    ($type:ty, $input:ty) => {
        impl<'de> serde::Deserialize<'de> for $type {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let value = <$input as serde::Deserialize>::deserialize(deserializer)?;

                Self::try_new(value)
                    .map_err(|err| <D::Error as serde::de::Error>::custom(err.message()))
            }
        }
    };
}

/// Implements `Deref` and `DerefMut` in the most common way they're used
#[macro_export]
macro_rules! impl_deref {
    ($type:ty, $target:ty) => {
        impl std::ops::Deref for $type {
            type Target = $target;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl std::ops::DerefMut for $type {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };
}

/// A shortcut for defining a newtype that implements `Deref` and `Deserialize`.
/// Also adds some common macros like deriving sqlx::Type, and the sqlx Transparent macro, which
/// can be turned off if not needed for the particular newtype
#[macro_export]
macro_rules! newtype {
    // Implementation
    (@impl
        derive = yes,
        deref = $deref:ty,
        deserialize = $deserialize:ty,

        $(#[$meta:meta])*
        $vis:vis struct $name:ident($inner:ty);
    ) => {
        #[derive(sqlx::Type, utoipa::ToSchema)]
        #[sqlx(transparent)]
        $(#[$meta])*
        $vis struct $name($inner);

        impl_deref!($name, $deref);
        impl_deserialize_via_try_new!($name, $deserialize);
    };

    (@impl
        derive = no,
        deref = $deref:ty,
        deserialize = $deserialize:ty,

        $(#[$meta:meta])*
        $vis:vis struct $name:ident($inner:ty);
    ) => {
        $(#[$meta])*
        $vis struct $name($inner);

        impl_deref!($name, $deref);
        impl_deserialize_via_try_new!($name, $deserialize);
    };

    // Public API

    // Default to inner type for Deref and Deserialize implementations, include extra derives
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident($inner:ty);
    ) => {
        newtype! {
            @impl
            derive = yes,
            deref = $inner,
            deserialize = $inner,

            $(#[$meta])*
            $vis struct $name($inner);
        }
    };

    // Default to inner types for Deref and Dserialize implementations, exclude extra derives
    (
        no_extra_derives

        $(#[$meta:meta])*
        $vis:vis struct $name:ident($inner:ty);
    ) => {
        newtype! {
            @impl
            derive = no,
            deref = $inner,
            deserialize = $inner,

            $(#[$meta])*
            $vis struct $name($inner);
        }
    };

    // Explicit types for Deref and Default implementations, include extra derives
    (
        deref_as($deref:ty)
        deserialize_as($deserialize:ty)

        $(#[$meta:meta])*
        $vis:vis struct $name:ident($inner:ty);
    ) => {
        newtype! {
            @impl
            derive = yes,
            deref = $deref,
            deserialize = $deserialize,

            $(#[$meta])*
            $vis struct $name($inner);
        }
    };

    // Explicit types for Deref and Default implementations, exclude extra derives
    (
        no_extra_derives

        deref_as($deref:ty)
        deserialize_as($deserialize:ty)

        $(#[$meta:meta])*
        $vis:vis struct $name:ident($inner:ty);
    ) => {
        newtype! {
            @impl
            derive = no,
            deref = $deref,
            deserialize = $deserialize,

            $(#[$meta])*
            $vis struct $name($inner);
        }
    };
}

/// An entity ID macro that generates common code for all entity IDs, which are newtypes around i64
#[macro_export]
macro_rules! entity_id {
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident;
    ) => {
        newtype! {
            /// An i64 newtype for all database entity IDs
            ///
            /// ## Methods
            /// - try_new(i64)
            ///
            /// ## Derives:
            /// - Debug
            /// - serde::Serialize
            /// - utoipa::ToSchema
            /// - sqlx::Type
            /// - Copy
            /// - Clone
            ///
            /// ## Implements
            /// - serde::Deserialize
            /// - Display
            ///
            /// ## Attributes
            /// - sqlx(transparent)
            $(#[$meta])*
            #[derive(Debug, serde::Serialize, Copy, Clone)]
            $vis struct $name(i64);
        }

        impl $name {
            pub fn try_new(value: i64) -> Result<Self, AppError> {
                Ok(Self(value))
            }
        }

        impl From<$name> for i64 {
            fn from(value: $name) -> Self {
                value.0
            }
        }

        impl_display_via_to_string!($name);
    };
}

/// Helper macro to implement sqlx traits, including Type, Decode, and Encode
/// Useful for wrapper types that have inner unsigned integer inner types, which are not supported
/// by sqlx::Type, sqlx::Decode and sqlx::Encode by default
#[macro_export]
macro_rules! impl_sqlx_type_via {
    ($ty:ty => $db_ty:ty) => {
        impl sqlx::Type<sqlx::Postgres> for $ty {
            fn type_info() -> sqlx::postgres::PgTypeInfo {
                <$db_ty as sqlx::Type<sqlx::Postgres>>::type_info()
            }

            fn compatible(ty: &sqlx::postgres::PgTypeInfo) -> bool {
                <$db_ty as sqlx::Type<sqlx::Postgres>>::compatible(ty)
            }
        }

        impl<'q> sqlx::Encode<'q, sqlx::Postgres> for $ty {
            fn encode_by_ref(
                &self,
                buf: &mut sqlx::postgres::PgArgumentBuffer,
            ) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
                let value: $db_ty = self.0.into();
                <$db_ty as sqlx::Encode<sqlx::Postgres>>::encode_by_ref(&value, buf)
            }

            fn size_hint(&self) -> usize {
                let value: $db_ty = self.0.into();
                <$db_ty as sqlx::Encode<sqlx::Postgres>>::size_hint(&value)
            }
        }

        impl<'r> sqlx::Decode<'r, sqlx::Postgres> for $ty {
            fn decode(
                value: sqlx::postgres::PgValueRef<'r>,
            ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
                let value = <$db_ty as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
                Ok(Self(value.try_into()?))
            }
        }
    };
}

/// Implements Display using the Display implementation of the inner type
/// Useful for newtype wrappers that wrap a primitive type that already implements Display
#[macro_export]
macro_rules! impl_display_via_to_string {
    ($name:ty) => {
        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                ::std::fmt::Display::fmt(&**self, f)
            }
        }
    };
}
