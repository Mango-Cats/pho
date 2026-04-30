/// Compute the base Jaro similarity between two character sequences.
///
/// The Jaro similarity considers:
/// - Matching characters (characters that are the same and within a certain distance)
/// - Transpositions (matching characters that are out of order)
pub(crate) fn jaro_similarity(x: &[char], y: &[char]) -> f32 {
    let x_length = x.len();
    let y_length = y.len();

    if x_length == 0 && y_length == 0 {
        return 1.0;
    }

    if x_length == 0 || y_length == 0 {
        return 0.0;
    }

    // Maximum allowed distance for matching characters
    let match_distance = (x_length.max(y_length) / 2).saturating_sub(1);

    let mut x_matches = vec![false; x_length];
    let mut y_matches = vec![false; y_length];

    let mut matching_characters = 0;
    let mut transpositions = 0;

    // Find matching characters
    for i in 0..x_length {
        let start = i.saturating_sub(match_distance);
        let end = (i + match_distance + 1).min(y_length);

        for j in start..end {
            if y_matches[j] || x[i] != y[j] {
                continue;
            }

            x_matches[i] = true;
            y_matches[j] = true;
            matching_characters += 1;
            break;
        }
    }

    if matching_characters == 0 {
        return 0.0;
    }

    // Count transpositions
    let mut y_position = 0;
    for i in 0..x_length {
        if !x_matches[i] {
            continue;
        }

        while !y_matches[y_position] {
            y_position += 1;
        }

        if x[i] != y[y_position] {
            transpositions += 1;
        }

        y_position += 1;
    }

    let matching_characters_f32 = matching_characters as f32;
    let transpositions_f32 = (transpositions / 2) as f32;

    // Jaro similarity formula
    (matching_characters_f32 / x_length as f32
        + matching_characters_f32 / y_length as f32
        + (matching_characters_f32 - transpositions_f32) / matching_characters_f32)
        / 3.0
}
