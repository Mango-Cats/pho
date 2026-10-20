use pho::{
    Error,
    algorithms::{
        Algorithm, Aline, Editex, JaroWinkler, Keyboard, Levenshtein, NeedlemanWunsch,
        SmithWaterman,
        aline::config::{Costs as AlineCosts, Salience},
        editex::config::Costs as EditexCosts,
        levenshtein::config::Costs as LevenshteinCosts,
    },
    utils::io::{export, import},
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
        AlineCosts::try_new(10.0, 35.0, 45.0, 5.0),
        Err(Error::InvalidAlineCost { name: "skip", .. })
    ));
    assert!(matches!(
        AlineCosts::try_new(-10.0, -35.0, 45.0, 5.0),
        Err(Error::InvalidAlineCost {
            name: "substitute",
            ..
        })
    ));
    assert!(AlineCosts::try_new(-10.0, 35.0, 45.0, 5.0).is_ok());
}

#[test]
fn aline_costs_reject_non_finite() {
    for bad in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
        assert!(matches!(
            AlineCosts::try_new(bad, 35.0, 45.0, 5.0),
            Err(Error::InvalidAlineCost { name: "skip", .. })
        ));
        assert!(matches!(
            AlineCosts::try_new(-10.0, 35.0, 45.0, bad),
            Err(Error::InvalidAlineCost {
                name: "vowel_consonant",
                ..
            })
        ));
    }
    assert!(AlineCosts::try_new(-10.5, 35.25, 45.0, 0.0).is_ok());
}

fn salience_with_place(place: f32) -> pho::Result<Salience> {
    Salience::try_new(
        5.0, place, 50.0, 5.0, 20.0, 10.0, 10.0, 5.0, 10.0, 3.0, 2.0, 2.0, 10.0, 10.0, 10.0, 0.0,
    )
}

#[test]
fn aline_salience_rejects_nan_infinite_and_negative() {
    for bad in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY, -1.0] {
        assert!(matches!(
            salience_with_place(bad),
            Err(Error::InvalidAlineSalience { name: "place", .. })
        ));
    }
    assert!(salience_with_place(0.0).is_ok());
    assert!(salience_with_place(22.21).is_ok());
}

const FIL_ALINE: &str = "algorithm_configs/fil/filipino_aline.toml";

/// The Filipino ALINE config with the line `from` replaced by `to`.
fn fil_aline_with(from: &str, to: &str) -> String {
    let text = std::fs::read_to_string(FIL_ALINE).unwrap();
    let lines: Vec<&str> = text.lines().collect();
    assert_eq!(lines.iter().filter(|l| **l == from).count(), 1, "{from}");
    lines
        .iter()
        .map(|l| if *l == from { to } else { l })
        .collect::<Vec<_>>()
        .join("\n")
}

#[test]
fn aline_toml_with_integer_weights_loads() {
    let aline: Aline = import(FIL_ALINE).unwrap();
    assert_eq!(aline.salience.place(), 40.0);
    assert_eq!(aline.salience.stress(), 0.0);
    assert_eq!(aline.costs.skip(), -10.0);
    assert_eq!(aline.costs.substitute(), 35.0);
}

#[test]
fn aline_toml_with_decimal_weights_loads() {
    let text = fil_aline_with("place = 40", "place = 22.5");
    let text = text.replace("skip = -10\n", "skip = -10.25\n");
    let aline: Aline = toml::from_str(&text).unwrap();
    assert_eq!(aline.salience.place(), 22.5);
    assert_eq!(aline.costs.skip(), -10.25);
}

#[test]
fn aline_toml_rejects_bad_salience_and_costs() {
    for bad in ["nan", "inf", "-inf", "-1.0"] {
        let text = fil_aline_with("place = 40", &format!("place = {bad}"));
        let err = toml::from_str::<Aline>(&text).unwrap_err();
        assert!(err.to_string().contains("salience place"), "{bad}: {err}");
    }
    for bad in ["nan", "inf", "-inf"] {
        let text = fil_aline_with("skip = -10", &format!("skip = {bad}"));
        let err = toml::from_str::<Aline>(&text).unwrap_err();
        assert!(err.to_string().contains("cost skip"), "{bad}: {err}");
    }
    let text = fil_aline_with("skip = -10", "skip = 1.5");
    let err = toml::from_str::<Aline>(&text).unwrap_err();
    assert!(err.to_string().contains("non-positive"), "{err}");
}

#[test]
fn aline_round_trip_keeps_non_integer_weights() {
    let mut aline: Aline = import(FIL_ALINE).unwrap();
    aline.set_salience(salience_with_place(22.21).unwrap());
    aline.set_costs(AlineCosts::try_new(-9.75, 35.5, 44.125, 4.2).unwrap());

    let path = format!("{}/aline_round_trip.toml", env!("CARGO_TARGET_TMPDIR"));
    export(&path, &aline, false).unwrap();
    let back: Aline = import(&path).unwrap();

    assert_eq!(back.salience.place(), 22.21);
    assert_eq!(back.costs.skip(), -9.75);
    assert_eq!(back.costs.substitute(), 35.5);
    assert_eq!(back.costs.expand_compress(), 44.125);
    assert_eq!(back.costs.vowel_consonant(), 4.2);
    assert_eq!(
        toml::to_string(&back.salience).unwrap(),
        toml::to_string(&aline.salience).unwrap()
    );
    assert_eq!(
        back.similarity("pa", "ba").unwrap(),
        aline.similarity("pa", "ba").unwrap()
    );
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
