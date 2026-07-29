use super::config::VisualWeighted;
use super::edit::{delete, replace};

fn build_matrix(x: &[char], y: &[char], config: &VisualWeighted) -> (Vec<f32>, usize, usize) {
    let m = x.len();
    let n = y.len();

    let mut d = vec![0.0f32; (m + 1) * (n + 1)];
    let idx = |i: usize, j: usize| -> usize { i * (n + 1) + j };

    for i in 1..=m {
        let previous = if i >= 2 { Some(x[i - 2]) } else { None };
        d[idx(i, 0)] = d[idx(i - 1, 0)] + delete(x[i - 1], previous, config);
    }

    for j in 1..=n {
        let previous = if j >= 2 { Some(y[j - 2]) } else { None };
        d[idx(0, j)] = d[idx(0, j - 1)] + delete(y[j - 1], previous, config);
    }

    for i in 1..=m {
        for j in 1..=n {
            let x_previous = if i >= 2 { Some(x[i - 2]) } else { None };
            let y_previous = if j >= 2 { Some(y[j - 2]) } else { None };
            let delete_score = d[idx(i - 1, j)] + delete(x[i - 1], x_previous, config);
            let insert_score = d[idx(i, j - 1)] + delete(y[j - 1], y_previous, config);
            let replace_score = d[idx(i - 1, j - 1)] + replace(x[i - 1], y[j - 1], config);

            d[idx(i, j)] = delete_score.min(insert_score).min(replace_score);
        }
    }

    (d, m, n)
}

/// Visual-weighted edit distance using substitution/insertion/deletion costs
/// drawn from a visual letter-confusability grouping.
pub fn distance(x: &[char], y: &[char], config: &VisualWeighted) -> f32 {
    let (d, m, n) = build_matrix(x, y, config);
    d[m * (n + 1) + n]
}

/// Traceback the minimal-cost alignment path and tally how many
/// substitutions, insertions, and deletions it uses (matches, where the
/// characters are equal, are not counted).
///
/// On ties between multiple minimal-cost moves at a cell, the diagonal
/// (match/substitution) move is preferred, then deletion, then insertion —
/// a deterministic rule chosen to minimize the total operation count.
pub fn operation_counts(x: &[char], y: &[char], config: &VisualWeighted) -> (u32, u32, u32) {
    let (d, m, n) = build_matrix(x, y, config);
    let idx = |i: usize, j: usize| -> usize { i * (n + 1) + j };

    let (mut substitutions, mut insertions, mut deletions) = (0u32, 0u32, 0u32);
    let (mut i, mut j) = (m, n);

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

        let current = d[idx(i, j)];
        let x_previous = if i >= 2 { Some(x[i - 2]) } else { None };
        let y_previous = if j >= 2 { Some(y[j - 2]) } else { None };
        let replace_score = d[idx(i - 1, j - 1)] + replace(x[i - 1], y[j - 1], config);
        let delete_score = d[idx(i - 1, j)] + delete(x[i - 1], x_previous, config);
        let insert_score = d[idx(i, j - 1)] + delete(y[j - 1], y_previous, config);

        if replace_score == current {
            if x[i - 1] != y[j - 1] {
                substitutions += 1;
            }
            i -= 1;
            j -= 1;
        } else if delete_score == current {
            deletions += 1;
            i -= 1;
        } else {
            debug_assert_eq!(insert_score, current);
            insertions += 1;
            j -= 1;
        }
    }

    (substitutions, insertions, deletions)
}

pub(super) fn total_delete_cost(chars: &[char], config: &VisualWeighted) -> f32 {
    let mut total = 0.0;

    for (idx, symbol) in chars.iter().enumerate() {
        let previous = if idx == 0 { None } else { Some(chars[idx - 1]) };
        total += delete(*symbol, previous, config);
    }

    total
}
