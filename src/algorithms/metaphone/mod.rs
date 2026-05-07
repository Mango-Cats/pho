//! metaphone
//!
//! Original Metaphone phonetic similarity (Lawrence Philips, 1990).
//!
//! Both strings are reduced to a phonetic skeleton via English pronunciation
//! rules and then compared with Jaro similarity on the resulting codes.
//! Unlike Soundex, Metaphone produces variable-length codes that capture more
//! phonetic detail, making it better suited for longer drug names.

pub mod config;
mod encode;

use crate::{algorithms::Algorithm, error::Result};
use config::Metaphone;
use encode::metaphone;

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

fn truncate(code: &str, max_len: usize) -> String {
    if max_len == 0 {
        return code.to_string();
    }
    code.chars().take(max_len).collect()
}

impl Algorithm for Metaphone {
    fn similarity(&self, x: &str, y: &str) -> Result<f32> {
        let code_x = truncate(&metaphone(x), self.max_code_length);
        let code_y = truncate(&metaphone(y), self.max_code_length);

        if code_x.is_empty() || code_y.is_empty() {
            return Ok(if code_x == code_y { 1.0 } else { 0.0 });
        }

        let cx: Vec<char> = code_x.chars().collect();
        let cy: Vec<char> = code_y.chars().collect();
        Ok(jaro(&cx, &cy))
    }
}
