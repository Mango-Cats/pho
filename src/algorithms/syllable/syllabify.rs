fn is_vowel(c: char) -> bool {
    matches!(c.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u' | 'y')
}

/// Maximal-onset syllabification for English/drug names.
///
/// Strategy:
/// 1. Identify vowel-cluster nuclei.
/// 2. Between two nuclei, split at the largest onset cluster the following
///    nucleus can legally take (CV, CCV, etc.).
/// 3. The remaining consonants attach to the coda of the preceding syllable.
///
/// Returns a list of syllables as strings (lowercase).
pub fn syllabify(word: &str) -> Vec<String> {
    let chars: Vec<char> = word.to_lowercase().chars().collect();
    let n = chars.len();

    if n == 0 {
        return Vec::new();
    }

    // Find positions of vowel groups (each group = one nucleus).
    let mut nuclei: Vec<(usize, usize)> = Vec::new(); // (start, end) exclusive
    let mut i = 0;
    while i < n {
        if is_vowel(chars[i]) {
            let start = i;
            while i < n && is_vowel(chars[i]) {
                i += 1;
            }
            nuclei.push((start, i));
        } else {
            i += 1;
        }
    }

    if nuclei.is_empty() {
        // All consonants — treat as one syllable.
        return vec![chars.iter().collect()];
    }

    // Build syllable boundaries.
    // boundary[k] = index of first char belonging to syllable k.
    let mut boundaries: Vec<usize> = vec![0];

    for k in 1..nuclei.len() {
        let prev_end = nuclei[k - 1].1; // end of previous nucleus
        let curr_start = nuclei[k].0; // start of current nucleus
        let consonants: Vec<char> = chars[prev_end..curr_start].to_vec();
        let onset = maximal_onset(&consonants);
        // Split: coda comes from prev_end to (curr_start - onset).
        let boundary = curr_start - onset;
        boundaries.push(boundary.max(prev_end)); // don't go before the vowel end
    }

    // Build syllable strings.
    boundaries.push(n);
    let mut syllables: Vec<String> = Vec::with_capacity(nuclei.len());
    for k in 0..boundaries.len() - 1 {
        let s: String = chars[boundaries[k]..boundaries[k + 1]].iter().collect();
        if !s.is_empty() {
            syllables.push(s);
        }
    }

    syllables
}

/// Returns how many consonants from the end of `consonants` can legally
/// begin an English syllable (maximal onset principle).
///
/// Legal English onsets used in drug names: any single consonant,
/// common two-consonant clusters (bl, br, cl, cr, dr, fl, fr, gl, gr,
/// pl, pr, sc, sk, sl, sm, sn, sp, st, str, sw, th, tr, tw, wh),
/// three-consonant: str, spl, spr.
fn maximal_onset(consonants: &[char]) -> usize {
    let n = consonants.len();
    if n == 0 {
        return 0;
    }

    // Try three-consonant onset from the tail.
    if n >= 3 {
        let tri: String = consonants[n - 3..].iter().collect();
        if matches!(tri.as_str(), "str" | "spl" | "spr" | "scr" | "shr" | "thr") {
            return 3;
        }
    }

    // Try two-consonant onset.
    if n >= 2 {
        let di: String = consonants[n - 2..].iter().collect();
        if matches!(
            di.as_str(),
            "bl" | "br" | "cl" | "cr" | "dr" | "fl" | "fr" | "gl" | "gr" | "pl" | "pr"
            | "sc" | "sk" | "sl" | "sm" | "sn" | "sp" | "st" | "sw"
            | "th" | "tr" | "tw" | "wh" | "ph" | "ch" | "sh"
            | "kl" | "kr" | "kn" | "wr" | "gn" | "pn" | "tz"
        ) {
            return 2;
        }
    }

    // Single consonant: always valid onset.
    1
}

/// Extract syllable bigrams as pairs of consecutive syllable strings.
pub fn syllable_bigrams(word: &str) -> Vec<(String, String)> {
    let sylls = syllabify(word);
    sylls.windows(2).map(|w| (w[0].clone(), w[1].clone())).collect()
}
