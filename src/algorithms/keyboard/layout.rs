/// QWERTY key positions as (row, col) with fractional offsets per row.
///
/// Rows 1-3 are staggered: row 1 shifts right by 0.5, row 2 by 1.0, row 3
/// by 1.5 — matching the physical layout so diagonal neighbours are ~1.0
/// apart in Euclidean space.
fn key_position(c: char) -> Option<(f32, f32)> {
    let c = c.to_ascii_lowercase();
    let pos: Option<(usize, usize)> = match c {
        '`' => Some((0, 0)),
        '1' => Some((0, 1)),
        '2' => Some((0, 2)),
        '3' => Some((0, 3)),
        '4' => Some((0, 4)),
        '5' => Some((0, 5)),
        '6' => Some((0, 6)),
        '7' => Some((0, 7)),
        '8' => Some((0, 8)),
        '9' => Some((0, 9)),
        '0' => Some((0, 10)),
        '-' => Some((0, 11)),
        '=' => Some((0, 12)),

        'q' => Some((1, 0)),
        'w' => Some((1, 1)),
        'e' => Some((1, 2)),
        'r' => Some((1, 3)),
        't' => Some((1, 4)),
        'y' => Some((1, 5)),
        'u' => Some((1, 6)),
        'i' => Some((1, 7)),
        'o' => Some((1, 8)),
        'p' => Some((1, 9)),

        'a' => Some((2, 0)),
        's' => Some((2, 1)),
        'd' => Some((2, 2)),
        'f' => Some((2, 3)),
        'g' => Some((2, 4)),
        'h' => Some((2, 5)),
        'j' => Some((2, 6)),
        'k' => Some((2, 7)),
        'l' => Some((2, 8)),

        'z' => Some((3, 0)),
        'x' => Some((3, 1)),
        'c' => Some((3, 2)),
        'v' => Some((3, 3)),
        'b' => Some((3, 4)),
        'n' => Some((3, 5)),
        'm' => Some((3, 6)),
        _ => None,
    };

    pos.map(|(row, col)| {
        let stagger = row as f32 * 0.5;
        (row as f32, col as f32 + stagger)
    })
}

/// Euclidean distance between two keys on the QWERTY layout.
/// Returns None if either character is not in the layout.
pub fn key_distance(a: char, b: char) -> Option<f32> {
    let (ar, ac) = key_position(a)?;
    let (br, bc) = key_position(b)?;
    let dr = ar - br;
    let dc = ac - bc;
    Some((dr * dr + dc * dc).sqrt())
}

/// Maximum possible key distance on the layout (approx `q` to `m`).
pub const MAX_KEY_DISTANCE: f32 = 8.0;
