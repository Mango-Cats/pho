//! double_metaphone
//!
//! Double Metaphone phonetic similarity (Lawrence Philips, 2000).
//!
//! Each string produces a *primary* and *secondary* code, accounting for
//! multiple European-origin pronunciations. The similarity is the maximum
//! Jaro score across all four primary/secondary code pairings — ensuring
//! that a match on either variant counts.

pub mod config;
mod encode;

use crate::{algorithms::Algorithm, error::Result};
use config::DoubleMetaphone;
use encode::double_metaphone;

fn jaro(a: &[char], b: &[char]) -> f32 {
    let m_len = a.len();
    let n_len = b.len();

    if m_len == 0 && n_len == 0 {
        return 1.0;
    }
    if m_len == 0 || n_len == 0 {
        return 0.0;
    }

    let window = (m_len.max(n_len) / 2).saturating_sub(1);
    let mut a_matched = vec![false; m_len];
    let mut b_matched = vec![false; n_len];
    let mut matches = 0usize;

    for i in 0..m_len {
        let lo = i.saturating_sub(window);
        let hi = (i + window + 1).min(n_len);
        for j in lo..hi {
            if !b_matched[j] && a[i] == b[j] {
                a_matched[i] = true;
                b_matched[j] = true;
                matches += 1;
                break;
            }
        }
    }

    if matches == 0 {
        return 0.0;
    }

    let mut transpositions = 0usize;
    let mut k = 0;
    for i in 0..m_len {
        if !a_matched[i] {
            continue;
        }
        while !b_matched[k] {
            k += 1;
        }
        if a[i] != b[k] {
            transpositions += 1;
        }
        k += 1;
    }

    let m = matches as f32;
    let t = (transpositions / 2) as f32;
    (m / m_len as f32 + m / n_len as f32 + (m - t) / m) / 3.0
}

fn code_chars(code: &str, max_len: usize) -> Vec<char> {
    if max_len == 0 {
        code.chars().collect()
    } else {
        code.chars().take(max_len).collect()
    }
}

fn jaro_codes(a: &[char], b: &[char]) -> f32 {
    if a.is_empty() || b.is_empty() {
        return if a == b { 1.0 } else { 0.0 };
    }
    jaro(a, b)
}

impl Algorithm for DoubleMetaphone {
    fn similarity(&self, x: &str, y: &str) -> Result<f32> {
        let (xp, xs) = double_metaphone(x);
        let (yp, ys) = double_metaphone(y);

        let max = self.max_code_length;
        let xp = code_chars(&xp, max);
        let xs = code_chars(&xs, max);
        let yp = code_chars(&yp, max);
        let ys = code_chars(&ys, max);

        // Maximum similarity across all four pairings.
        let score = jaro_codes(&xp, &yp)
            .max(jaro_codes(&xp, &ys))
            .max(jaro_codes(&xs, &yp))
            .max(jaro_codes(&xs, &ys));

        Ok(score.clamp(0.0, 1.0))
    }
}
