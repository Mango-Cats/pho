use pho::algorithms::{
    config_io::parse_toml_file,
    editex::{
        self,
        config::EditexConfig,
        edit::{delete, replace},
    },
};

const TOML_PATH: &str = "tests/config_sample_editex.toml";

fn load() -> EditexConfig {
    match parse_toml_file(TOML_PATH) {
        Ok(config) => config,
        Err(e) => panic!("Can't open {TOML_PATH}: {e}."),
    }
}

fn assert_approx(actual: f32, expected: f32, tol: f32, word1: &str, word2: &str) {
    let diff = (actual - expected).abs();
    assert!(
        diff <= tol,
        "{word1} ~ {word2}: expected {expected} ± {tol}, got {actual} (diff {diff})"
    );
}

fn tokenize_and_validate(
    input: &str,
    config: &EditexConfig,
    input_name: &'static str,
) -> Vec<char> {
    let chars: Vec<char> = input.chars().map(|c| c.to_ascii_lowercase()).collect();

    for (idx, symbol) in chars.iter().enumerate() {
        if !config.group.contains_key(symbol) {
            panic!("unknown token '{symbol}' at position {idx} in {input_name} for Editex config");
        }
    }

    chars
}

fn edit_distance(x: &[char], y: &[char], config: &EditexConfig) -> f32 {
    let m = x.len();
    let n = y.len();

    let mut d = vec![0.0f32; (m + 1) * (n + 1)];
    let idx = |i: usize, j: usize| -> usize { i * (n + 1) + j };

    for i in 1..=m {
        let previous = if i >= 2 { Some(x[i - 2]) } else { None };
        d[idx(i, 0)] = d[idx(i - 1, 0)] + delete(x[i - 1], previous, config);
    }

    for j in 1..=n {
        let previous = if j >= 2 { Some(y[j - 2]) } else { None };
        d[idx(0, j)] = d[idx(0, j - 1)] + delete(y[j - 1], previous, config);
    }

    for i in 1..=m {
        for j in 1..=n {
            let x_previous = if i >= 2 { Some(x[i - 2]) } else { None };
            let y_previous = if j >= 2 { Some(y[j - 2]) } else { None };
            let delete_score = d[idx(i - 1, j)] + delete(x[i - 1], x_previous, config);
            let insert_score = d[idx(i, j - 1)] + delete(y[j - 1], y_previous, config);
            let replace_score = d[idx(i - 1, j - 1)] + replace(x[i - 1], y[j - 1], config);

            d[idx(i, j)] = delete_score.min(insert_score).min(replace_score);
        }
    }

    d[idx(m, n)]
}

#[test]
fn matches_reference_editex_distances() {
    let config = load();
    let tol = 1e-6;

    const CASES: &[(&str, &str, f32)] = &[
        ("Smith", "Smyth", 1.0),
        ("Catherine", "Katherine", 1.0),
        ("Brian", "Bryan", 1.0),
        ("Calendar", "Calender", 1.0),
        ("Similarity", "Simularity", 1.0),
        ("Color", "Colour", 1.0),
        ("Relevant", "Relevante", 2.0),
        ("Knight", "Night", 2.0),
        ("Stephen", "Steven", 3.0),
        ("Paxil", "Taxol", 3.0),
        ("Accept", "Except", 3.0),
        ("Zantac", "Xanax", 5.0),
        ("Physics", "Fizziks", 6.0),
        ("Apple", "Banana", 9.0),
    ];

    for &(w1, w2, expected) in CASES {
        let x_chars = tokenize_and_validate(w1, &config, "word1");
        let y_chars = tokenize_and_validate(w2, &config, "word2");
        let actual = edit_distance(&x_chars, &y_chars, &config);
        assert_approx(actual, expected, tol, w1, w2);
    }
}

#[test]
fn wrapper_similarity_is_in_range() {
    let config = load();
    let sim = editex::similarity("Smith", "Smyth", &config).unwrap();
    assert!((0.0..=1.0).contains(&sim));
}
