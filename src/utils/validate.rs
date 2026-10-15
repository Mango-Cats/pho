// src/utils/validate.rs

/// Validates tokens from an iterator against a closure.
///
/// Returns a vector of tokens if all are valid, or [`crate::Error::UnknownToken`]
/// on the first invalid token.
pub(crate) fn validate_tokens<T, I, F>(
    tokens: I,
    input_name: &'static str,
    context: &'static str,
    is_valid: F,
) -> crate::Result<Vec<T>>
where
    T: ToString,
    I: IntoIterator<Item = T>,
    F: Fn(&T) -> bool,
{
    let mut validated = Vec::new();

    for (position, token) in tokens.into_iter().enumerate() {
        if !is_valid(&token) {
            return Err(crate::Error::UnknownToken {
                token: token.to_string(),
                position,
                input_name,
                context,
            });
        }
        validated.push(token);
    }

    Ok(validated)
}

/// Checks that a cost-like value is finite and non-negative.
///
/// Returns [`crate::Error::InvalidCost`] naming the offending field otherwise.
pub(crate) fn non_negative(name: &'static str, value: f32) -> crate::Result<()> {
    if value.is_finite() && value >= 0.0 {
        Ok(())
    } else {
        Err(crate::Error::InvalidCost { name, value })
    }
}

/// Implements `Serialize` and `Deserialize` for a config type whose derives
/// carry `#[serde(remote = "Self")]`, running the type's `validate()` after
/// deserializing. This way configs loaded from TOML go through the same
/// checks as `try_new` and the setters.
macro_rules! validated_serde {
    ($ty:ty) => {
        impl<'de> ::serde::Deserialize<'de> for $ty {
            fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, D::Error>
            where
                D: ::serde::Deserializer<'de>,
            {
                let value = <$ty>::deserialize(deserializer)?;
                value.validate().map_err(::serde::de::Error::custom)?;
                Ok(value)
            }
        }

        impl ::serde::Serialize for $ty {
            fn serialize<S>(&self, serializer: S) -> ::core::result::Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,
            {
                <$ty>::serialize(self, serializer)
            }
        }
    };
}

pub(crate) use validated_serde;
