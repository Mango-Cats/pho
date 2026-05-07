use super::matrix::DrugNameMatrix;

/// Needleman-Wunsch global alignment score using the supplied matrix.
pub fn nw_score(x: &[char], y: &[char], gap: f32, matrix: &DrugNameMatrix) -> f32 {
    let m = x.len();
    let n = y.len();

    let idx = |i: usize, j: usize| i * (n + 1) + j;
    let mut dp = vec![0.0f32; (m + 1) * (n + 1)];

    for i in 1..=m {
        dp[idx(i, 0)] = dp[idx(i - 1, 0)] - gap;
    }
    for j in 1..=n {
        dp[idx(0, j)] = dp[idx(0, j - 1)] - gap;
    }

    for i in 1..=m {
        for j in 1..=n {
            let diag = dp[idx(i - 1, j - 1)] + matrix.score(x[i - 1], y[j - 1]);
            let del = dp[idx(i - 1, j)] - gap;
            let ins = dp[idx(i, j - 1)] - gap;
            dp[idx(i, j)] = diag.max(del).max(ins);
        }
    }

    dp[idx(m, n)]
}
