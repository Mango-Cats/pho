fn is_vowel(c: char) -> bool {
    matches!(c, 'A' | 'E' | 'I' | 'O' | 'U')
}

fn at(chars: &[char], i: usize) -> char {
    chars.get(i).copied().unwrap_or('\0')
}

/// Original Metaphone (Philips, 1990).
///
/// Converts an English word into a phonetic skeleton by applying a cascade
/// of letter-group rules. The result is a string of consonant codes that
/// represent the pronunciation of the word.
pub fn metaphone(s: &str) -> String {
    let chars: Vec<char> = s
        .to_uppercase()
        .chars()
        .filter(|c| c.is_alphabetic())
        .collect();

    if chars.is_empty() {
        return String::new();
    }

    // Drop silent initial pairs.
    let start = {
        let pair = (chars[0], at(&chars, 1));
        match pair {
            ('A', 'E') | ('G', 'N') | ('K', 'N') | ('P', 'N') | ('W', 'R') => 1,
            _ => 0,
        }
    };

    let mut out = String::new();
    let mut i = start;

    while i < chars.len() {
        let c = chars[i];
        let prev = if i > 0 { at(&chars, i - 1) } else { '\0' };
        let next = at(&chars, i + 1);
        let next2 = at(&chars, i + 2);

        // Skip duplicate adjacent letters except C.
        if c != 'C' && prev == c {
            i += 1;
            continue;
        }

        match c {
            // Vowels: keep only at start of word.
            'A' | 'E' | 'I' | 'O' | 'U' => {
                if i == start {
                    out.push(c);
                }
            }

            'B' => {
                // Silent after M at end.
                if !(prev == 'M' && i + 1 == chars.len()) {
                    out.push('B');
                }
            }

            'C' => {
                if next == 'I' && next2 == 'A' || next == 'I' && next2 == 'O' {
                    out.push('X'); // CIA, CIO → X
                } else if next == 'H' {
                    out.push('X'); // CH → X
                    i += 1;
                } else if next == 'I' || next == 'E' || next == 'Y' {
                    out.push('S'); // CE, CI, CY → S
                } else if next == 'K' {
                    i += 1; // CK → skip K (C already handled below)
                    // Do nothing: CK reduces to K, but we already output nothing for C.
                    // Actually let's output K here.
                    out.push('K');
                } else if prev == 'S' && (next == 'I' || next == 'E' || next == 'Y') {
                    // SCI/SCE/SCY → S already captured, skip C.
                } else {
                    out.push('K');
                }
            }

            'D' => {
                if next == 'G' && (next2 == 'E' || next2 == 'I' || next2 == 'Y') {
                    out.push('J'); // DGE, DGI, DGY → J
                    i += 1;
                } else {
                    out.push('T');
                }
            }

            'F' => out.push('F'),

            'G' => {
                let is_silent = (next == 'H' && !is_vowel(next2))
                    || (next == 'N' && (i + 1 == chars.len() || (next2 == 'E' && i + 3 == chars.len())))
                    || (prev == 'D' && (next == 'E' || next == 'I' || next == 'Y'));

                if !is_silent {
                    if next == 'H' {
                        // GH before vowel → K
                        if is_vowel(next2) {
                            out.push('K');
                        }
                        i += 1;
                    } else if next == 'E' || next == 'I' || next == 'Y' {
                        out.push('J');
                    } else if prev != 'G' {
                        out.push('K');
                    }
                } else if next == 'H' {
                    i += 1; // skip the H
                }
            }

            'H' => {
                // Keep H only when at start or preceded by a vowel AND followed by vowel.
                if is_vowel(next) && (i == 0 || !is_vowel(prev)) {
                    out.push('H');
                }
            }

            'J' => out.push('J'),
            'K' => {
                if prev != 'C' {
                    out.push('K');
                }
            }
            'L' => out.push('L'),
            'M' => out.push('M'),
            'N' => out.push('N'),

            'P' => {
                if next == 'H' {
                    out.push('F');
                    i += 1;
                } else {
                    out.push('P');
                }
            }

            'Q' => out.push('K'),
            'R' => out.push('R'),

            'S' => {
                if next == 'H' || (next == 'I' && (next2 == 'O' || next2 == 'A')) {
                    out.push('X');
                    if next == 'H' {
                        i += 1;
                    }
                } else {
                    out.push('S');
                }
            }

            'T' => {
                if next == 'H' {
                    out.push('0'); // TH → theta (represented as '0')
                    i += 1;
                } else if next == 'I' && (next2 == 'A' || next2 == 'O') {
                    out.push('X');
                } else if next != 'C' || at(&chars, i + 2) != 'H' {
                    out.push('T');
                }
            }

            'V' => out.push('F'),
            'W' => {
                // W only when at start or before a vowel.
                if i == 0 || is_vowel(next) {
                    out.push('W');
                }
            }
            'X' => {
                out.push('K');
                out.push('S');
            }
            'Y' => {
                if is_vowel(next) {
                    out.push('Y');
                }
            }
            'Z' => out.push('S'),

            _ => {}
        }

        i += 1;
    }

    out
}
