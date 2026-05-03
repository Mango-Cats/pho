// src/utils/validation.rs

use crate::errors::{AlgorithmError, UnknownTokenError};

/// Consumes an iterator of tokens, validating each one against a provided closure.
/// Returns a collected `Vec<T>` of the tokens if all are valid, or an `AlgorithmError`
/// on the first invalid token.
pub(crate) fn validate_tokens<T, I, F>(
    tokens: I,
    input_name: &'static str,
    context: &'static str,
    is_valid: F,
) -> Result<Vec<T>, AlgorithmError>
where
    T: ToString,
    I: IntoIterator<Item = T>,
    F: Fn(&T) -> bool,
{
    let mut validated = Vec::new();

    for (position, token) in tokens.into_iter().enumerate() {
        if !is_valid(&token) {
            return Err(AlgorithmError::UnknownTokenError(UnknownTokenError {
                token: token.to_string(),
                position,
                input_name,
                context,
            }));
        }
        validated.push(token);
    }

    Ok(validated)
}
