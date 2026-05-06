use super::config::Editex;
use super::edit::{delete, replace};

/// Editex distance using substitution/insertion/deletion costs.
pub fn distance(x: &[char], y: &[char], config: &Editex) -> f32 {
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

    d[idx(m, n)]
}

pub(super) fn total_delete_cost(chars: &[char], config: &Editex) -> f32 {
    let mut total = 0.0;

    for (idx, symbol) in chars.iter().enumerate() {
        let previous = if idx == 0 { None } else { Some(chars[idx - 1]) };
        total += delete(*symbol, previous, config);
    }

    total
}
