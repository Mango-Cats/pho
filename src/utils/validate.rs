// src/utils/validation.rs

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
