/// Double Metaphone (Lawrence Philips, 2000).
///
/// Returns `(primary, secondary)` phonetic codes. The secondary code captures
/// an alternative pronunciation (e.g. Germanic vs Romance origin).
/// Both codes are non-empty strings of consonant symbols.
pub fn double_metaphone(s: &str) -> (String, String) {
    let chars: Vec<char> = s
        .to_uppercase()
        .chars()
        .filter(|c| c.is_alphabetic())
        .collect();

    if chars.is_empty() {
        return (String::new(), String::new());
    }

    let mut primary = String::new();
    let mut secondary = String::new();

    let at = |i: usize| -> char { chars.get(i).copied().unwrap_or('\0') };
    let slice_eq = |start: usize, s: &str| -> bool {
        let bytes: Vec<char> = s.chars().collect();
        chars
            .get(start..start + bytes.len())
            .map_or(false, |sl| sl == bytes.as_slice())
    };
    let is_vowel_char = |c: char| matches!(c, 'A' | 'E' | 'I' | 'O' | 'U' | 'Y');

    let mut i: usize = 0;

    // Skip initial silent letters.
    if slice_eq(0, "GN")
        || slice_eq(0, "KN")
        || slice_eq(0, "PN")
        || slice_eq(0, "AE")
        || slice_eq(0, "WR")
    {
        i = 1;
    }

    // Initial vowel → A.
    if i == 0 && is_vowel_char(at(0)) {
        primary.push('A');
        secondary.push('A');
        i = 1;
    }

    let add = |p: &mut String, s: &mut String, pa: &str, sa: &str| {
        p.push_str(pa);
        s.push_str(if sa.is_empty() { pa } else { sa });
    };

    macro_rules! push {
        ($pa:expr) => {
            add(&mut primary, &mut secondary, $pa, "")
        };
        ($pa:expr, $sa:expr) => {
            add(&mut primary, &mut secondary, $pa, $sa)
        };
    }

    while i < chars.len() {
        let c = at(i);

        match c {
            'A' | 'E' | 'I' | 'O' | 'U' | 'Y' => {
                i += 1;
                continue;
            }

            'B' => {
                push!("P");
                // Skip silent B after M at end.
                i += if at(i + 1) == 'B' { 2 } else { 1 };
                continue;
            }

            '\u{00C7}' => {
                push!("S");
                i += 1;
                continue;
            } // Ç

            'C' => {
                if i > 1
                    && !is_vowel_char(at(i - 2))
                    && slice_eq(i - 1, "ACH")
                    && at(i + 2) != 'I'
                    && (at(i + 2) != 'E' || slice_eq(i - 2, "BACHER") || slice_eq(i - 2, "MACHER"))
                {
                    push!("K");
                    i += 2;
                    continue;
                }
                if i == 0 && slice_eq(0, "CAESAR") {
                    push!("S");
                    i += 2;
                    continue;
                }
                if slice_eq(i, "CHIA") {
                    push!("K");
                    i += 2;
                    continue;
                }
                if slice_eq(i, "CH") {
                    if i > 0 && slice_eq(i, "CHAE") {
                        push!("K", "X");
                        i += 2;
                        continue;
                    }
                    // Initial CH: Germanic → K, else X.
                    let germanic = i == 0 && (slice_eq(i + 2, "AE") || is_vowel_char(at(i + 2)));
                    if germanic {
                        push!("K");
                    } else {
                        push!("X");
                    }
                    i += 2;
                    continue;
                }
                if slice_eq(i, "CZ") && !slice_eq(i.saturating_sub(2), "WICZ") {
                    push!("S", "X");
                    i += 2;
                    continue;
                }
                if slice_eq(i + 1, "IA") {
                    push!("X");
                    i += 2;
                    continue;
                }
                if slice_eq(i, "CC") && !(i == 1 && at(0) == 'M') {
                    if at(i + 2) == 'I' || at(i + 2) == 'E' || at(i + 2) == 'H' {
                        if (i == 1 && at(0) == 'A')
                            || (i > 0 && (slice_eq(i - 1, "UCCEE") || slice_eq(i - 1, "UCCES")))
                        {
                            push!("KS");
                        } else {
                            push!("X");
                        }
                        i += 3;
                    } else {
                        push!("K");
                        i += 2;
                    }
                    continue;
                }
                if slice_eq(i, "CK") || slice_eq(i, "CG") || slice_eq(i, "CQ") {
                    push!("K");
                    i += 2;
                    continue;
                }
                if slice_eq(i, "CI") || slice_eq(i, "CE") || slice_eq(i, "CY") {
                    if slice_eq(i, "CIO") || slice_eq(i, "CIA") || slice_eq(i, "CIE") {
                        push!("S", "X");
                    } else {
                        push!("S");
                    }
                    i += 2;
                    continue;
                }
                push!("K");
                if slice_eq(i + 1, " C") || slice_eq(i + 1, " Q") || slice_eq(i + 1, " G") {
                    i += 3;
                } else if (at(i + 1) == 'C' || at(i + 1) == 'K') && !slice_eq(i + 1, "CE") {
                    i += 2;
                } else {
                    i += 1;
                }
                continue;
            }

            'D' => {
                if slice_eq(i, "DG") {
                    if at(i + 2) == 'I' || at(i + 2) == 'E' || at(i + 2) == 'Y' {
                        push!("J");
                        i += 3;
                    } else {
                        push!("TK");
                        i += 2;
                    }
                } else if slice_eq(i, "DT") || slice_eq(i, "DD") {
                    push!("T");
                    i += 2;
                } else {
                    push!("T");
                    i += 1;
                }
                continue;
            }

            'F' => {
                push!("F");
                i += if at(i + 1) == 'F' { 2 } else { 1 };
                continue;
            }

            'G' => {
                if at(i + 1) == 'H' {
                    if i > 0 && !is_vowel_char(at(i - 1)) {
                        push!("K");
                        i += 2;
                        continue;
                    }
                    if i == 0 {
                        if at(i + 2) == 'I' {
                            push!("J");
                        } else {
                            push!("K");
                        }
                        i += 2;
                        continue;
                    }
                    if (i > 1 && (at(i - 2) == 'B' || at(i - 2) == 'H' || at(i - 2) == 'D'))
                        || (i > 2 && (at(i - 3) == 'B' || at(i - 3) == 'H' || at(i - 3) == 'D'))
                        || (i > 3 && (at(i - 4) == 'B' || at(i - 4) == 'H'))
                    {
                        i += 2;
                        continue;
                    }
                    if i > 2
                        && at(i - 1) == 'U'
                        && (at(i - 3) == 'C'
                            || at(i - 3) == 'G'
                            || at(i - 3) == 'L'
                            || at(i - 3) == 'R'
                            || at(i - 3) == 'T')
                    {
                        push!("F");
                    } else if i > 0 && at(i - 1) != 'I' {
                        push!("K");
                    }
                    i += 2;
                    continue;
                }
                if at(i + 1) == 'N' {
                    if i == 1 && is_vowel_char(at(0)) {
                        push!("KN", "N");
                    } else if !slice_eq(i + 2, "EY") && at(i + 1) != 'Y' {
                        push!("N");
                    } else {
                        push!("KN");
                    }
                    i += 2;
                    continue;
                }
                if slice_eq(i + 1, "LI") {
                    push!("KL", "L");
                    i += 2;
                    continue;
                }
                if i == 0
                    && (at(i + 1) == 'E'
                        || at(i + 1) == 'I'
                        || at(i + 1) == 'Y'
                        || slice_eq(i + 1, "ES")
                        || slice_eq(i + 1, "EP")
                        || slice_eq(i + 1, "EB")
                        || slice_eq(i + 1, "EL")
                        || slice_eq(i + 1, "EY")
                        || slice_eq(i + 1, "IB")
                        || slice_eq(i + 1, "IL")
                        || slice_eq(i + 1, "IN")
                        || slice_eq(i + 1, "IE")
                        || slice_eq(i + 1, "EI")
                        || slice_eq(i + 1, "ER"))
                {
                    push!("K", "J");
                    i += 2;
                    continue;
                }
                if (slice_eq(i + 1, "ER") || at(i + 1) == 'Y')
                    && !slice_eq(0, "DANGER")
                    && !slice_eq(0, "RANGER")
                    && !slice_eq(0, "MANGER")
                    && i > 0
                    && !matches!(at(i - 1), 'E' | 'I')
                    && !slice_eq(i - 1, "RGY")
                    && !slice_eq(i - 1, "OGY")
                {
                    push!("K", "J");
                    i += 2;
                    continue;
                }
                if at(i + 1) == 'E'
                    || at(i + 1) == 'I'
                    || at(i + 1) == 'Y'
                    || (i > 0 && (slice_eq(i - 1, "AGGI") || slice_eq(i - 1, "OGGI")))
                {
                    push!("J");
                } else {
                    push!("K");
                }
                i += if at(i + 1) == 'G' { 2 } else { 1 };
                continue;
            }

            'H' => {
                if (i == 0 || is_vowel_char(at(i - 1))) && is_vowel_char(at(i + 1)) {
                    push!("H");
                    i += 2;
                } else {
                    i += 1;
                }
                continue;
            }

            'J' => {
                if slice_eq(i, "JOSE") || slice_eq(0, "SAN ") {
                    push!("H", "J");
                } else if i == 0 {
                    push!("J", "A");
                } else if i > 0 && !is_vowel_char(at(i - 1)) && !slice_eq(0, "SAN ") {
                    push!("J");
                } else {
                    push!("Y", "J");
                }
                i += if at(i + 1) == 'J' { 2 } else { 1 };
                continue;
            }

            'K' => {
                push!("K");
                i += if at(i + 1) == 'K' { 2 } else { 1 };
                continue;
            }

            'L' => {
                if at(i + 1) == 'L' {
                    if (i > 0
                        && i == chars.len() - 3
                        && matches!(
                            (at(i - 1), at(i + 2)),
                            ('A', 'A')
                                | ('A', 'O')
                                | ('A', 'E')
                                | ('I', 'O')
                                | ('I', 'E')
                                | ('I', 'A')
                                | ('O', 'E')
                                | ('U', 'E')
                        ))
                        || (i > 0
                            && at(i - 1) == 'A'
                            && (i + 2 == chars.len() || matches!(at(i + 2), 'A' | 'O' | 'E')))
                    {
                        push!("L", "");
                    } else {
                        push!("L");
                    }
                    i += 2;
                } else {
                    push!("L");
                    i += 1;
                }
                continue;
            }

            'M' => {
                push!("M");
                i += if at(i + 1) == 'M' { 2 } else { 1 };
                continue;
            }

            'N' => {
                push!("N");
                i += if at(i + 1) == 'N' { 2 } else { 1 };
                continue;
            }

            '\u{00D1}' => {
                push!("N");
                i += 1;
                continue;
            } // Ñ

            'P' => {
                if at(i + 1) == 'H' {
                    push!("F");
                    i += 2;
                } else {
                    push!("P");
                    i += if at(i + 1) == 'P' { 2 } else { 1 };
                }
                continue;
            }

            'Q' => {
                push!("K");
                i += if at(i + 1) == 'Q' { 2 } else { 1 };
                continue;
            }

            'R' => {
                // French-origin: silent final R after E.
                if i + 1 == chars.len() && !slice_eq(0, "GN") && !slice_eq(0, "KN") {
                    push!("R", "");
                } else {
                    push!("R");
                }
                i += if at(i + 1) == 'R' { 2 } else { 1 };
                continue;
            }

            'S' => {
                if i > 0 && (slice_eq(i - 1, "ISL") || slice_eq(i - 1, "YSL")) {
                    i += 1;
                    continue;
                }
                if i == 0 && slice_eq(0, "SUGAR") {
                    push!("X", "S");
                    i += 1;
                    continue;
                }
                if slice_eq(i, "SH") {
                    push!("X");
                    i += 2;
                    continue;
                }
                if slice_eq(i, "SIO") || slice_eq(i, "SIA") {
                    push!("S", "X");
                    i += 3;
                    continue;
                }
                let initial_s_exception = i == 0
                    && (slice_eq(i + 1, "M")
                        || slice_eq(i + 1, "N")
                        || slice_eq(i + 1, "L")
                        || slice_eq(i + 1, "W"));
                if slice_eq(i, "SC") {
                    if at(i + 2) == 'H' {
                        push!("SK");
                    } else if at(i + 2) == 'I' || at(i + 2) == 'E' || at(i + 2) == 'Y' {
                        push!("S");
                    } else {
                        push!("SK");
                    }
                    i += 3;
                    continue;
                }
                if i + 1 == chars.len() && i > 1 && (slice_eq(i - 2, "AI") || slice_eq(i - 2, "OI"))
                {
                    push!("S", "");
                } else {
                    push!("S");
                }
                i += if !initial_s_exception && (at(i + 1) == 'S' || at(i + 1) == 'Z') {
                    2
                } else {
                    1
                };
                continue;
            }

            'T' => {
                if slice_eq(i, "TION") || slice_eq(i, "TIA") || slice_eq(i, "TCH") {
                    push!("X");
                    i += if slice_eq(i, "TCH") { 3 } else { 3 };
                    continue;
                }
                if slice_eq(i, "TH") || slice_eq(i, "TTH") {
                    push!("0");
                    i += 2;
                    continue;
                }
                push!("T");
                i += if at(i + 1) == 'T' || at(i + 1) == 'D' {
                    2
                } else {
                    1
                };
                continue;
            }

            'V' => {
                push!("F");
                i += if at(i + 1) == 'V' { 2 } else { 1 };
                continue;
            }

            'W' => {
                if slice_eq(i, "WR") {
                    push!("R");
                    i += 2;
                    continue;
                }
                if i == 0 && (is_vowel_char(at(i + 1)) || slice_eq(i, "WH")) {
                    push!("A", "F");
                    i += 1;
                    continue;
                }
                // Slavic initial W.
                if (i + 1 == chars.len() || !is_vowel_char(at(i + 1)))
                    && !(i > 0 && is_vowel_char(at(i - 1)))
                {
                    push!("", "F");
                }
                i += 1;
                continue;
            }

            'X' => {
                if !(i + 1 == chars.len()
                    && ((i > 2 && (slice_eq(i - 3, "IAU") || slice_eq(i - 3, "EAU")))
                        || (i > 1 && (slice_eq(i - 2, "AU") || slice_eq(i - 2, "OU")))))
                {
                    push!("KS");
                }
                i += if at(i + 1) == 'C' || at(i + 1) == 'X' {
                    2
                } else {
                    1
                };
                continue;
            }

            'Z' => {
                if at(i + 1) == 'H' {
                    push!("J");
                    i += 2;
                    continue;
                }
                if slice_eq(i + 1, "ZO")
                    || slice_eq(i + 1, "ZI")
                    || slice_eq(i + 1, "ZA")
                    || (i > 0 && at(i - 1) == 'T' && i + 1 != chars.len())
                {
                    push!("S", "TS");
                } else {
                    push!("S");
                }
                i += if at(i + 1) == 'Z' { 2 } else { 1 };
                continue;
            }

            _ => {
                i += 1;
                continue;
            }
        }
    }

    (primary, secondary)
}
