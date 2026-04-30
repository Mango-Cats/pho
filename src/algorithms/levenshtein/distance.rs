use super::config::LevenshteinConfig;

/// Compute the Levenshtein edit distance between two character sequences.
///
/// Uses a dynamic programming table where `distance[i][j]` represents the
/// minimum cost to transform `x[0..i]` into `y[0..j]`.
pub(crate) fn edit_distance(x: &[char], y: &[char], config: &LevenshteinConfig) -> f32 {
    let x_length = x.len();
    let y_length = y.len();

    let mut distance = vec![0.0f32; (x_length + 1) * (y_length + 1)];
    let index = |i: usize, j: usize| -> usize { i * (y_length + 1) + j };

    for j in 1..=y_length {
        distance[index(0, j)] = distance[index(0, j - 1)] + config.costs.insert;
    }

    for i in 1..=x_length {
        distance[index(i, 0)] = distance[index(i - 1, 0)] + config.costs.delete;
    }

    for i in 1..=x_length {
        for j in 1..=y_length {
            let deletion_cost = distance[index(i - 1, j)] + config.costs.delete;
            let insertion_cost = distance[index(i, j - 1)] + config.costs.insert;

            let substitution_cost = if x[i - 1] == y[j - 1] {
                distance[index(i - 1, j - 1)]
            } else {
                distance[index(i - 1, j - 1)] + config.costs.substitute
            };

            distance[index(i, j)] = deletion_cost.min(insertion_cost).min(substitution_cost);
        }
    }

    distance[index(x_length, y_length)]
}
