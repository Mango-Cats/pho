use crate::algorithms::UnknownTokenError;

use super::config::EditexConfig;

/// Convert input into lowercase ASCII chars and validate each exists in the
/// configured groups.
pub(super) fn tokenize_and_validate(
    input: &str,
    config: &EditexConfig,
    input_name: &'static str,
) -> Result<Vec<char>, UnknownTokenError> {
    let chars: Vec<char> = input.chars().map(|c| c.to_ascii_lowercase()).collect();

    for (idx, symbol) in chars.iter().enumerate() {
        if !config.group.contains_key(symbol) {
            return Err(UnknownTokenError {
                token: symbol.to_string(),
                position: idx,
                input_name,
                context: "Editex config groups",
            });
        }
    }

    Ok(chars)
}
