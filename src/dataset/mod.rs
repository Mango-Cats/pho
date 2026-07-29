pub mod row;
pub mod score_matrix;

pub use row::{Row, RowBuilder, SplitConfig};

pub use score_matrix::ScoreMatrix;
#[cfg(test)]
mod tests {
    use crate::algorithms::Prefix;
    use crate::dataset::{Row, ScoreMatrix};
    use crate::ensemble::config::EnsembleConfig;
    use crate::ensemble::types::EnsembleAlgorithm;
    use crate::ensemble::weighted_function::WeightedFunction;
    use crate::{algorithms::Algorithm, error::Result};

    struct RequiresTranscription;

    impl Algorithm for RequiresTranscription {
        fn similarity(&self, x: &str, y: &str) -> Result<f32> {
            if x.is_empty() && y.is_empty() {
                return Ok(1.0);
            }
            Ok(0.5)
        }

        fn requires_transcription(&self) -> bool {
            true
        }

        fn name(&self) -> &'static str {
            "RequiresTranscription"
        }
    }

    #[test]
    fn from_slice_rejects_transcription_required_algorithm() {
        let algorithms: Vec<Box<dyn Algorithm>> = vec![Box::new(RequiresTranscription)];
        let rows = vec![Row::builder("a", "b").label(0.5f32).build()];

        let result = ScoreMatrix::from_slice(algorithms, &rows, false, false);
        assert!(result.is_err());
    }

    #[test]
    fn from_slice_accepts_transcription_required_algorithm() {
        let algorithms: Vec<Box<dyn Algorithm>> = vec![Box::new(RequiresTranscription)];
        let rows = vec![
            Row::builder("word", "ward")
                .label(0.5)
                .transcriptions("wɜd", "wɔɹd")
                .build(),
        ];

        let result = ScoreMatrix::from_slice(algorithms, &rows, false, false);
        assert!(result.is_ok());
    }

    #[test]
    fn from_slice_allows_missing_label() {
        let algorithms: Vec<Box<dyn Algorithm>> = vec![Box::new(RequiresTranscription)];
        let rows = vec![
            Row::builder("word", "ward")
                .transcriptions("wɜd", "wɔɹd")
                .build(),
        ];

        let result = ScoreMatrix::from_slice(algorithms, &rows, false, false);
        assert!(result.is_ok());
        let dataset = result.unwrap();
        assert_eq!(dataset.labels, vec![None]);
    }

    #[test]
    fn from_slice_can_add_length_features_and_rename_prefix_column() {
        let algorithms: Vec<Box<dyn Algorithm>> = vec![Box::new(Prefix::new(false))];
        let rows = vec![Row::builder("abc", "ab").label(0.5).build()];

        let dataset = ScoreMatrix::from_slice(algorithms, &rows, true, false)
            .expect("dataset with length features");

        assert_eq!(
            dataset.algorithm_names,
            vec![
                "common_prefix_ratio",
                "len_x1",
                "len_x2",
                "len_min",
                "len_max",
                "len_diff",
                "len_ratio",
                "common_prefix_len",
                "common_suffix_len",
                "common_suffix_ratio",
                "first_mismatch_pos",
                "first_char_match",
            ]
        );

        let scores = &dataset.base_scores[0];
        assert_eq!(scores.len(), 12);
        assert!((scores[0] - 2.0 / 3.0).abs() < 1e-6);
        assert_eq!(scores[1], 3.0);
        assert_eq!(scores[2], 2.0);
        assert_eq!(scores[3], 2.0);
        assert_eq!(scores[4], 3.0);
        assert_eq!(scores[5], 1.0);
        assert!((scores[6] - 2.0 / 3.0).abs() < 1e-6);
        assert_eq!(scores[7], 2.0);
        assert_eq!(scores[8], 0.0);
        assert_eq!(scores[9], 0.0);
        assert_eq!(scores[10], 1.0);
        assert_eq!(scores[11], 1.0);
    }

    #[test]
    fn from_ensemble_includes_ensemble_and_weighted_component_scores() {
        let ensemble = EnsembleAlgorithm::try_new(
            vec![
                WeightedFunction::from_function("aline", 0.8, false, |_x, _y| Ok(0.0)),
                WeightedFunction::from_function("bisim", 0.1, false, |_x, _y| Ok(0.1)),
                WeightedFunction::from_function("editex", 0.1, false, |_x, _y| Ok(0.2)),
            ],
            EnsembleConfig::Linear,
        )
        .expect("valid ensemble");

        let rows = vec![Row::builder("drug_a", "drug_b").label(0.0).build()];
        let dataset = ScoreMatrix::from_ensemble(&ensemble, &rows, false, false)
            .expect("dataset from ensemble");

        assert_eq!(
            dataset.algorithm_names,
            vec!["ensemble", "aline_0.8", "bisim_0.1", "editex_0.1"]
        );

        let scores = &dataset.base_scores[0];
        assert_eq!(scores.len(), 4);
        assert!((scores[0] - 0.03).abs() < 1e-6);
        assert!((scores[1] - 0.0).abs() < 1e-6);
        assert!((scores[2] - 0.1).abs() < 1e-6);
        assert!((scores[3] - 0.2).abs() < 1e-6);
    }
}
