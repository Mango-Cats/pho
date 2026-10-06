use super::Segment;
use super::config::{AlignmentMode, Aline};
use super::scoring::{expansion_score, indel_score, substitution_score};

/// Raw optimal alignment score.
///
/// Each segment carries its stress level (0.0 none, 0.5 secondary, 1.0
/// primary) and an optional length override. In the Kondrak variant all
/// stress levels are 0.0, so the stress term contributes nothing.
///
/// - `AlignmentMode::Local` mirrors NLTK's `_align_score` DP (including
///   expansion/compression edits): cells are floored at 0 and the matrix
///   maximum is returned.
/// - `AlignmentMode::Global` initializes row 0 / column 0 with cumulative
///   indel scores, does not floor at 0, and returns `S[m][n]`. The result
///   may be negative.
pub(crate) fn alignment_score(x: &[Segment], y: &[Segment], config: &Aline) -> f32 {
    let m = x.len();
    let n = y.len();
    let global = config.alignment_mode == AlignmentMode::Global;
    let skip = indel_score(config);

    // Flattened (m+1) x (n+1) DP matrix. Initialized to 0.0.
    let mut s = vec![0.0f32; (m + 1) * (n + 1)];
    let idx = |i: usize, j: usize| -> usize { i * (n + 1) + j };

    if global {
        for i in 1..=m {
            s[idx(i, 0)] = i as f32 * skip;
        }
        for j in 1..=n {
            s[idx(0, j)] = j as f32 * skip;
        }
    }
    let floor = if global { f32::NEG_INFINITY } else { 0.0 };

    let mut best = 0.0f32;

    for i in 1..=m {
        for j in 1..=n {
            let xi = &x[i - 1];
            let yj = &y[j - 1];

            let delete_score = s[idx(i - 1, j)] + skip;
            let insert_score = s[idx(i, j - 1)] + skip;
            let substitute_score = s[idx(i - 1, j - 1)] + substitution_score(xi, yj, config);

            let expand_x_score = if i > 1 {
                s[idx(i - 2, j - 1)] + expansion_score(yj, &x[i - 2], xi, config)
            } else {
                f32::NEG_INFINITY
            };

            let expand_y_score = if j > 1 {
                s[idx(i - 1, j - 2)] + expansion_score(xi, &y[j - 2], yj, config)
            } else {
                f32::NEG_INFINITY
            };

            let cell = delete_score
                .max(insert_score)
                .max(substitute_score)
                .max(expand_x_score)
                .max(expand_y_score)
                .max(floor);

            s[idx(i, j)] = cell;
            best = best.max(cell);
        }
    }

    if global { s[idx(m, n)] } else { best }
}
