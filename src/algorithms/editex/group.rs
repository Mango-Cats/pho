use super::config::EditexConfig;

pub fn same_group(a: char, b: char, config: &EditexConfig) -> bool {
    let Some(a_groups) = config.group.get(&a) else {
        return false;
    };
    let Some(b_groups) = config.group.get(&b) else {
        return false;
    };

    a_groups.iter().any(|group| b_groups.contains(group))
}
