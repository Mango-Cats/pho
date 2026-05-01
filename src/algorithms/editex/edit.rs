use super::{config::Editex, group::same_group};

pub(super) fn replace(a: char, b: char, config: &Editex) -> f32 {
    if a == b {
        return 0.0;
    }

    if same_group(a, b, config) {
        config.costs.same
    } else {
        config.costs.diff
    }
}

pub(super) fn delete(current: char, previous: Option<char>, config: &Editex) -> f32 {
    let Some(previous) = previous else {
        return config.costs.diff;
    };

    if same_group(current, previous, config) {
        config.costs.same
    } else {
        config.costs.diff
    }
}
