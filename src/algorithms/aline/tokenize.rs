use crate::algorithms::UnknownTokenError;

use super::config::AlineConfig;

/// Split an IPA string into grapheme clusters, and validate each segment exists
/// in the configured inventory.
pub(crate) fn tokenize_and_validate(
    input: &str,
    config: &AlineConfig,
    input_name: &'static str,
) -> Result<Vec<String>, UnknownTokenError> {
    use unicode_segmentation::UnicodeSegmentation;

    let segments: Vec<String> = UnicodeSegmentation::graphemes(input, true)
        .map(str::to_string)
        .collect();

    for (idx, segment) in segments.iter().enumerate() {
        if !config.sounds.contains_key(segment) {
            return Err(UnknownTokenError {
                token: segment.clone(),
                position: idx,
                input_name,
                context: "ALINE config sound inventory",
            });
        }
    }

    Ok(segments)
}
