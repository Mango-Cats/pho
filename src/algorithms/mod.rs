pub mod aline;
pub mod editex;

pub enum SimilarityAlgorithm {
    Aline,
    Editex,
}

pub enum SimilarityAlgorithmConfig {
    AlineConfig,
    EditexConfig,
}

#[warn(dead_code)]
impl SimilarityAlgorithm {
    /// This is he shared similarity
    fn similarity(
        self,
        x: &'static str,
        y: &'static str,
        // this is option since some algorithms have a config (or a
        // set of variables) while others don't.
        similarity_config: Option<&SimilarityAlgorithmConfig>,
    ) -> f32 {
        match self {
            SimilarityAlgorithm::Aline => todo!(),
            SimilarityAlgorithm::Editex => todo!(),
            _ => panic!("`SimilarityAlgorithm` doesn't cover all possible cases"),
        }
        10.1
    }
}
