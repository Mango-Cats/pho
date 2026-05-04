//! String normalization utilities

/// Normalizes input string based on case sensitivity setting.
/// Returns a vector of characters from the normalized string.
pub fn normalize_input(input: &str, case_insensitive: bool) -> Vec<char> {
    let normalized = if case_insensitive {
        input.to_lowercase()
    } else {
        input.to_string()
    };

    normalized.chars().collect()
}
