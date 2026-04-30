use pho::{
    algorithms::{
        AlgorithmTrait, AlineAlgorithm,
        aline::{self, config::AlineConfig},
    },
    config_io::read,
};

const TOML_PATH: &str = "tests/config_sample_aline.toml";

fn load() -> AlineConfig {
    match read(TOML_PATH) {
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

#[test]
fn self_similarity_is_one() {
    let config = load();
    let score = aline::similarity("s", "s", &config).unwrap();
    assert_approx(score, 1.0, 1e-6, "s", "s");
}

#[test]
fn wrapper_self_similarity_is_one() {
    let config = load();
    let algo = AlineAlgorithm::new(config);
    let score = algo.similarity("s", "s").unwrap();
    assert_approx(score, 1.0, 1e-6, "s", "s");
}

#[test]
fn matches_nltk_reference_similarity_table() {
    // Reference scores are from the NLTK ALINE implementation (Kondrak 2002)
    // using the same costs/salience/feature matrix.
    let config = load();

    let tol = 1e-3;

    const CASES: &[(&str, &str, f32)] = &[
        ("jo", "ʒə", 0.7083_f32),
        ("tu", "ty", 0.9667_f32),
        ("nosotros", "nu", 0.2400_f32),
        ("kjen", "ki", 0.3846_f32),
        ("ke", "kwa", 0.5263_f32),
        ("todos", "tu", 0.3871_f32),
        ("una", "ən", 0.6706_f32),
        ("dos", "dø", 0.6105_f32),
        ("tres", "trwa", 0.6538_f32),
        ("ombre", "om", 0.3871_f32),
        ("arbol", "arbrə", 0.6903_f32),
        ("pluma", "plym", 0.8258_f32),
        ("kabeθa", "kap", 0.5000_f32),
        ("boka", "buʃ", 0.6792_f32),
        ("pje", "pje", 1.0000_f32),
        ("koraθon", "kœr", 0.4326_f32),
        ("ber", "vwar", 0.5808_f32),
        ("benir", "vənir", 0.9323_f32),
        ("deθir", "dir", 0.5194_f32),
        ("pobre", "povrə", 0.9323_f32),
        ("ðis", "dIzes", 0.6000_f32),
        ("ðæt", "das", 0.8211_f32),
        ("wat", "vas", 0.7684_f32),
        ("nat", "nixt", 0.6731_f32),
        ("loŋ", "laŋ", 0.9579_f32),
        ("mæn", "man", 1.0000_f32),
        ("fleʃ", "flajʃ", 0.7303_f32),
        ("bləd", "blyt", 0.9385_f32),
        ("feðər", "fEdər", 0.9387_f32),
        ("hær", "hAr", 1.0000_f32),
        ("ir", "Or", 0.9333_f32),
        ("aj", "awgə", 0.4000_f32),
        ("nowz", "nAzə", 0.6346_f32),
        ("mawθ", "munt", 0.5423_f32),
        ("təŋ", "tsuŋə", 0.6097_f32),
        ("fut", "fys", 0.9000_f32),
        ("nij", "knI", 0.6316_f32),
        ("hænd", "hant", 0.9615_f32),
        ("hart", "herts", 0.8030_f32),
        ("livər", "lEbər", 0.9387_f32),
        ("ænd", "ante", 0.7500_f32),
        ("æt", "ad", 0.9167_f32),
        ("blow", "flAre", 0.5839_f32),
        ("ir", "awris", 0.3226_f32),
        ("ijt", "edere", 0.3103_f32),
        ("fiʃ", "piʃkis", 0.4500_f32),
        ("flow", "fluere", 0.6389_f32),
        ("staɾ", "stella", 0.6184_f32),
        ("ful", "plenus", 0.2658_f32),
        ("græs", "gramen", 0.5000_f32),
        ("hart", "kordis", 0.4921_f32),
        ("horn", "korny", 0.6613_f32),
        ("aj", "ego", 0.4235_f32),
        ("nij", "genU", 0.4667_f32),
        ("məðər", "mAter", 0.8935_f32),
        ("mawntən", "mons", 0.4822_f32),
        ("nejm", "nomen", 0.5226_f32),
        ("njuw", "nowus", 0.5484_f32),
        ("wən", "unus", 0.4750_f32),
        ("rawnd", "rotundus", 0.4800_f32),
        ("sow", "suere", 0.5517_f32),
        ("sit", "sedere", 0.5000_f32),
        ("θrij", "tres", 0.7462_f32),
        ("tuwθ", "dentis", 0.4158_f32),
        ("θin", "tenwis", 0.4500_f32),
        ("kinwawa", "kenuaʔ", 0.5628_f32),
        ("nina", "nenah", 0.7742_f32),
        ("napewa", "napɛw", 0.8611_f32),
        ("wapimini", "wapemen", 0.8958_f32),
        ("namesa", "namɛʔs", 0.7632_f32),
        ("okimawa", "okemaw", 0.8780_f32),
        ("ʃiʃipa", "seʔsep", 0.7211_f32),
        ("ahkohkwa", "ahkɛh", 0.6040_f32),
        ("pematesiweni", "pematesewen", 0.9306_f32),
        ("asenja", "aʔsɛn", 0.6111_f32),
    ];

    for &(w1, w2, expected) in CASES {
        let actual = aline::similarity(w1, w2, &config).unwrap();
        assert_approx(actual, expected, tol, w1, w2);
    }
}
