/// Drug-name-tuned character substitution matrix.
///
/// Scores reflect English phonetic confusability relevant to pharmaceutical
/// naming conventions, similar in spirit to BLOSUM but for drug names:
///
/// - Same character: +2
/// - Phonetically close pair: +1 (e.g. f/v, d/t, b/p, m/n, s/z, c/k, i/y)
/// - Unrelated: −1
///
/// The matrix is symmetric and 26 × 26 (a=0 … z=25).
pub struct DrugNameMatrix {
    data: [[f32; 26]; 26],
}

impl DrugNameMatrix {
    pub fn new() -> Self {
        let mut data = [[-1.0f32; 26]; 26];

        // Diagonal: exact match.
        for i in 0..26 {
            data[i][i] = 2.0;
        }

        // Phonetically / graphically confusable pairs relevant to drug names.
        let similar: &[&[u8]] = &[
            b"bp",   // bilabial stops
            b"dt",   // alveolar stops
            b"fv",   // labiodentals
            b"mn",   // nasals
            b"sz",   // sibilants
            b"ck",   // velars (c-as-k vs k)
            b"gj",   // voiced palatal/velar
            b"lr",   // liquids
            b"iy",   // high front vowels
            b"ae",   // low front vowels (common drug suffix confusion)
            b"ou",   // back vowels
            b"ei",   // mid/high front vowels
            b"uo",   // rounded back vowels
            b"cq",   // both map to /k/
            b"xz",   // both end in /z/ in many drug names
            b"ph",   // 'ph' is often /f/; single 'h' vs 'f' confusion
        ];

        for group in similar {
            for &a in group.iter() {
                for &b in group.iter() {
                    if a != b {
                        let ai = (a - b'a') as usize;
                        let bi = (b - b'a') as usize;
                        data[ai][bi] = 1.0;
                        data[bi][ai] = 1.0;
                    }
                }
            }
        }

        Self { data }
    }

    #[inline]
    pub fn score(&self, a: char, b: char) -> f32 {
        let a = a.to_ascii_lowercase();
        let b = b.to_ascii_lowercase();
        if a.is_ascii_alphabetic() && b.is_ascii_alphabetic() {
            let ai = (a as u8 - b'a') as usize;
            let bi = (b as u8 - b'a') as usize;
            self.data[ai][bi]
        } else if a == b {
            2.0
        } else {
            -1.0
        }
    }

    /// Self-alignment score for a string — the maximum achievable score.
    pub fn self_score(&self, chars: &[char]) -> f32 {
        chars.iter().map(|&c| self.score(c, c)).sum()
    }
}

impl Default for DrugNameMatrix {
    fn default() -> Self {
        Self::new()
    }
}
