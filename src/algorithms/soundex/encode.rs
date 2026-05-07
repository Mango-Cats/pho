fn soundex_code(c: char) -> char {
    match c {
        'B' | 'F' | 'P' | 'V' => '1',
        'C' | 'G' | 'J' | 'K' | 'Q' | 'S' | 'X' | 'Z' => '2',
        'D' | 'T' => '3',
        'L' => '4',
        'M' | 'N' => '5',
        'R' => '6',
        _ => '0',
    }
}

/// American Soundex: returns a four-character code (letter + three digits).
///
/// H and W are transparent (do not break adjacency of same-coded consonants).
/// Vowels break adjacency (separating two consonants with the same code).
pub fn soundex(s: &str) -> String {
    let chars: Vec<char> = s
        .to_uppercase()
        .chars()
        .filter(|c| c.is_alphabetic())
        .collect();

    if chars.is_empty() {
        return String::new();
    }

    let first = chars[0];
    let mut result = String::with_capacity(4);
    result.push(first);

    // Start prev_code as the code of the first letter so that a letter with
    // the same code immediately after the first is suppressed (e.g. "Pfister").
    let mut prev_code = soundex_code(first);

    for &c in &chars[1..] {
        if c == 'H' || c == 'W' {
            // Transparent — do not update prev_code.
            continue;
        }

        let code = soundex_code(c);

        if code == '0' {
            // Vowel: separates adjacent same-coded consonants on either side.
            prev_code = '0';
        } else if code != prev_code {
            result.push(code);
            prev_code = code;
            if result.len() == 4 {
                break;
            }
        }
        // Same code as previous → suppress.
    }

    while result.len() < 4 {
        result.push('0');
    }

    result
}

/// Position-wise agreement between two four-character Soundex codes.
pub fn code_similarity(a: &str, b: &str) -> f32 {
    let matches = a.chars().zip(b.chars()).filter(|(ca, cb)| ca == cb).count();
    matches as f32 / 4.0
}
