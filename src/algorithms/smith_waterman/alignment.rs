use super::config::SmithWaterman;

/// Smith-Waterman local alignment score.
///
/// The matrix floor at 0.0 means only locally similar subsequences
/// accumulate score; the best cell gives the optimal local alignment.
pub fn sw_score(x: &[char], y: &[char], config: &SmithWaterman) -> f32 {
    let m = x.len();
    let n = y.len();

    if m == 0 || n == 0 {
        return 0.0;
    }

    let idx = |i: usize, j: usize| i * (n + 1) + j;
    let mut dp = vec![0.0f32; (m + 1) * (n + 1)];
    let mut best = 0.0f32;

    for i in 1..=m {
        for j in 1..=n {
            let sub = if x[i - 1] == y[j - 1] {
                dp[idx(i - 1, j - 1)] + config.match_score
            } else {
                dp[idx(i - 1, j - 1)] - config.mismatch_penalty
            };

            let del = dp[idx(i - 1, j)] - config.gap_penalty;
            let ins = dp[idx(i, j - 1)] - config.gap_penalty;

            let cell = sub.max(del).max(ins).max(0.0);
            dp[idx(i, j)] = cell;

            if cell > best {
                best = cell;
            }
        }
    }

    best
}
