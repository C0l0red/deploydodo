/// Generates a database entity and an associated struct without the ID, to allow for easier creation of new entities
#[macro_export]
macro_rules! entity {
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident {
            id: $id_ty:ty,
            $(
                $field:ident : $ty:ty
            ),* $(,)?
        }
    ) => {
        $(#[$meta])*
        $vis struct $name {
            pub id: $id_ty,
            $(
                pub $field: $ty,
            )*
        }

        paste::paste! {
            $(#[$meta])*
            $vis struct [<New $name>] {
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

/// Implements `Deref` in the most common way it's used
#[macro_export]
macro_rules! impl_deref {
    ($type:ty, $target:ty) => {
        impl std::ops::Deref for $type {
            type Target = $target;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    };
}

/// A shortcut for defining a newtype that implements `Deref` and `Deserialize`.
#[macro_export]
macro_rules! newtype {
    // Neither deref_as nor deserialize_as specified
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident($inner:ty);
    ) => {
        $(#[$meta])*
        $vis struct $name($inner);

        impl_deref!($name, $inner);
        impl_deserialize_via_try_new!($name, $inner);
    };

    // Only deserialize_as
    (
        deserialize_as($deserialize:ty)

        $(#[$meta:meta])*
        $vis:vis struct $name:ident($inner:ty);
    ) => {
        $(#[$meta])*
        $vis struct $name($inner);

        impl_deref!($name, $inner);
        impl_deserialize_via_try_new!($name, $deserialize);
    };

    // Only deref_as
    (
        deref_as($deref:ty)

        $(#[$meta:meta])*
        $vis:vis struct $name:ident($inner:ty);
    ) => {
        $(#[$meta])*
        $vis struct $name($inner);

        impl_deref!($name, $deref);
        impl_deserialize_via_try_new!($name, $inner);
    };

    // Both specified (either order would require another arm)
    (
        deserialize_as($deserialize:ty)
        deref_as($deref:ty)

        $(#[$meta:meta])*
        $vis:vis struct $name:ident($inner:ty);
    ) => {
        $(#[$meta])*
        $vis struct $name($inner);

        impl_deref!($name, $deref);
        impl_deserialize_via_try_new!($name, $deserialize);
    };
}

