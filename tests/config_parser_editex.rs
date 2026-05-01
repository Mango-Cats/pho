#[cfg(test)]
mod tests {
    use core::panic;

    use pho::{algorithms::Editex, config_io::import};

    const TOML_PATH: &str = "tests/config_sample_editex.toml";

    fn load() -> Editex {
        match import(TOML_PATH) {
            Ok(config) => config,
            Err(e) => panic!("Can't open {TOML_PATH}: {e}."),
        }
    }

    #[test]
    fn cost_same() {
        assert_eq!(load().costs.same, 1.0);
    }

    #[test]
    fn cost_diff() {
        assert_eq!(load().costs.diff, 2.0);
    }

    #[test]
    fn group_has_expected_size() {
        assert_eq!(load().group.len(), 25);
    }

    #[test]
    fn group_a() {
        assert_eq!(load().group[&'a'], vec![0]);
    }

    #[test]
    fn group_c() {
        assert_eq!(load().group[&'c'], vec![2, 9]);
    }

    #[test]
    fn group_h() {
        assert_eq!(load().group[&'h'], vec![0]);
    }

    #[test]
    fn group_p() {
        assert_eq!(load().group[&'p'], vec![1, 7]);
    }

    #[test]
    fn group_z() {
        assert_eq!(load().group[&'z'], vec![8, 9]);
    }

    #[test]
    fn rejects_non_toml_extension() {
        let result: Result<Editex, String> = import("notatoml.json");
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "file must be a .toml");
    }

    #[test]
    fn rejects_missing_file() {
        let result: Result<Editex, String> = import("nonexistent.toml");
        assert!(result.is_err());
    }
}
