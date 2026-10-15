use pho::{
    Error,
    algorithms::{
        Aline, Editex, JaroWinkler, Keyboard, Levenshtein, NeedlemanWunsch, SmithWaterman,
        aline::config::Costs as AlineCosts, editex::config::Costs as EditexCosts,
        levenshtein::config::Costs as LevenshteinCosts,
    },
    utils::io::import,
};

#[test]
fn levenshtein_costs_reject_negative_and_nan() {
    assert!(matches!(
        LevenshteinCosts::try_new(-1.0, 1.0, 1.0),
        Err(Error::InvalidCost {
            name: "costs.insert",
            ..
        })
    ));
    assert!(LevenshteinCosts::try_new(1.0, f32::NAN, 1.0).is_err());
    assert!(LevenshteinCosts::try_new(0.0, 0.0, 0.0).is_ok());
}

#[test]
fn editex_costs_require_same_not_above_diff() {
    assert!(matches!(
        EditexCosts::try_new(2.0, 1.0),
        Err(Error::InvalidCostOrder { .. })
    ));
    assert!(EditexCosts::try_new(1.0, 1.0).is_ok());
    assert!(EditexCosts::try_new(-0.5, 1.0).is_err());
}

#[test]
fn aline_costs_check_signs() {
    assert!(matches!(
        AlineCosts::try_new(10, 35, 45, 5),
        Err(Error::InvalidAlineCost { name: "skip", .. })
    ));
    assert!(matches!(
        AlineCosts::try_new(-10, -35, 45, 5),
        Err(Error::InvalidAlineCost {
            name: "substitute",
            ..
        })
    ));
    assert!(AlineCosts::try_new(-10, 35, 45, 5).is_ok());
}

#[test]
fn rejected_setter_leaves_config_unchanged() {
    let mut keyboard = Keyboard::try_new(1.0, 1.0, 1.0, false).unwrap();
    assert!(keyboard.set_insert_cost(-2.0).is_err());
    assert!(keyboard.set_insert_cost(3.0).is_ok());

    let mut sw = SmithWaterman::try_new(2.0, 1.0, 2.0, false).unwrap();
    assert!(matches!(
        sw.set_match_score(0.0),
        Err(Error::InvalidMatchScore(_))
    ));
    assert!(sw.set_gap_penalty(f32::INFINITY).is_err());

    let mut nw = NeedlemanWunsch::try_new(1.0, false).unwrap();
    assert!(nw.set_gap_penalty(-1.0).is_err());
}

#[test]
fn aline_epsilon_rejects_nan() {
    let mut aline: Aline = import("algorithm_configs/eng/aline.toml").unwrap();
    assert!(matches!(
        aline.set_epsilon(f32::NAN),
        Err(Error::NegativeEpsilon(_))
    ));
    assert!(aline.set_epsilon(0.0).is_ok());
}

#[test]
fn shipped_configs_pass_validation() {
    let _: Aline = import("algorithm_configs/eng/aline.toml").unwrap();
    let _: Editex = import("algorithm_configs/eng/editex.toml").unwrap();
    let _: Levenshtein = import("algorithm_configs/eng/levenshtein.toml").unwrap();
    let _: Keyboard = import("algorithm_configs/eng/keyboard.toml").unwrap();
    let _: SmithWaterman = import("algorithm_configs/eng/smith_waterman.toml").unwrap();
    let _: NeedlemanWunsch = import("algorithm_configs/eng/needleman_wunsch.toml").unwrap();
    let _: JaroWinkler = import("algorithm_configs/eng/jaro_winkler.toml").unwrap();
}

#[test]
fn deserialization_runs_validation() {
    let bad_keyboard = "insert_cost = -1.0\ndelete_cost = 1.0\nsubstitution_scale = 1.0\ncase_insensitive = false\n";
    assert!(toml::from_str::<Keyboard>(bad_keyboard).is_err());

    let bad_editex = "[costs]\nsame = 3.0\ndiff = 2.0\n\n[group]\n";
    let err = toml::from_str::<Editex>(bad_editex).unwrap_err();
    assert!(err.to_string().contains("must not exceed"), "{err}");

    let bad_jw = "prefix_scale = 0.5\nmax_prefix_length = 4\ncase_insensitive = false\n";
    assert!(toml::from_str::<JaroWinkler>(bad_jw).is_err());
}

#[test]
fn round_trip_serialization_still_works() {
    let keyboard = Keyboard::try_new(1.0, 2.0, 0.5, true).unwrap();
    let text = toml::to_string(&keyboard).unwrap();
    let back: Keyboard = toml::from_str(&text).unwrap();
    assert_eq!(toml::to_string(&back).unwrap(), text);
}
