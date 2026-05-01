use super::config::Aline;
use super::scoring::{expansion_score, indel_score, substitution_score};

/// Raw optimal local alignment score.
///
/// Mirrors NLTK's `_align_score` DP, including expansion/compression edits.
pub(crate) fn alignment_score(x: &[String], y: &[String], config: &Aline) -> f32 {
    let m = x.len();
    let n = y.len();

    // Flattened (m+1) x (n+1) DP matrix. Initialized to 0.0.
    let mut s = vec![0.0f32; (m + 1) * (n + 1)];
    let idx = |i: usize, j: usize| -> usize { i * (n + 1) + j };

    let mut best = 0.0f32;

    for i in 1..=m {
        for j in 1..=n {
            let delete_score = s[idx(i - 1, j)] + indel_score(config);
            let insert_score = s[idx(i, j - 1)] + indel_score(config);
            let substitute_score =
                s[idx(i - 1, j - 1)] + substitution_score(&x[i - 1], &y[j - 1], config);

            let expand_x_score = if i > 1 {
                s[idx(i - 2, j - 1)] + expansion_score(&y[j - 1], &x[i - 2], &x[i - 1], config)
            } else {
                f32::NEG_INFINITY
            };

            let expand_y_score = if j > 1 {
                s[idx(i - 1, j - 2)] + expansion_score(&x[i - 1], &y[j - 2], &y[j - 1], config)
            } else {
                f32::NEG_INFINITY
            };

            let cell = delete_score
                .max(insert_score)
                .max(substitute_score)
                .max(expand_x_score)
                .max(expand_y_score)
                .max(0.0);

            s[idx(i, j)] = cell;
            best = best.max(cell);
        }
    }

    best
}
