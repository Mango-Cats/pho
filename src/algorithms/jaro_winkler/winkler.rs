/// Compute the length of the common prefix between two strings.
///
/// Returns the number of matching characters at the beginning of both
/// strings, up to `max_length`.
pub(crate) fn common_prefix_length(x: &[char], y: &[char], max_length: usize) -> usize {
    let mut prefix_length = 0;
    let limit = x.len().min(y.len()).min(max_length);

    for i in 0..limit {
        if x[i] == y[i] {
            prefix_length += 1;
        } else {
            break;
        }
    }

    prefix_length
}
