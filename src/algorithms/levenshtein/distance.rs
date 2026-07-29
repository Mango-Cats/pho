use super::config::Levenshtein;

fn build_matrix(x: &[char], y: &[char], config: &Levenshtein) -> (Vec<f32>, usize, usize) {
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

    (distance, x_length, y_length)
}

/// Compute the Levenshtein edit distance between two character sequences.
///
/// Uses a dynamic programming table where `distance[i][j]` represents the
/// minimum cost to transform `x[0..i]` into `y[0..j]`.
pub fn distance(x: &[char], y: &[char], config: &Levenshtein) -> f32 {
    let (distance, x_length, y_length) = build_matrix(x, y, config);
    distance[x_length * (y_length + 1) + y_length]
}

/// Traceback the minimal-cost alignment path and tally how many
/// substitutions, insertions, and deletions it uses (matches, where the
/// characters are equal, are not counted).
///
/// On ties between multiple minimal-cost moves at a cell, the diagonal
/// (match/substitution) move is preferred, then deletion, then insertion —
/// a deterministic rule chosen to minimize the total operation count.
pub fn operation_counts(x: &[char], y: &[char], config: &Levenshtein) -> (u32, u32, u32) {
    let (distance, x_length, y_length) = build_matrix(x, y, config);
    let index = |i: usize, j: usize| -> usize { i * (y_length + 1) + j };

    let (mut substitutions, mut insertions, mut deletions) = (0u32, 0u32, 0u32);
    let (mut i, mut j) = (x_length, y_length);

    while i > 0 || j > 0 {
        if i == 0 {
            insertions += 1;
            j -= 1;
            continue;
        }
        if j == 0 {
            deletions += 1;
            i -= 1;
            continue;
        }

        let current = distance[index(i, j)];
        let substitution_cost = if x[i - 1] == y[j - 1] {
            distance[index(i - 1, j - 1)]
        } else {
            distance[index(i - 1, j - 1)] + config.costs.substitute
        };
        let deletion_cost = distance[index(i - 1, j)] + config.costs.delete;
        let insertion_cost = distance[index(i, j - 1)] + config.costs.insert;

        if substitution_cost == current {
            if x[i - 1] != y[j - 1] {
                substitutions += 1;
            }
            i -= 1;
            j -= 1;
        } else if deletion_cost == current {
            deletions += 1;
            i -= 1;
        } else {
            debug_assert_eq!(insertion_cost, current);
            insertions += 1;
            j -= 1;
        }
    }

    (substitutions, insertions, deletions)
}
