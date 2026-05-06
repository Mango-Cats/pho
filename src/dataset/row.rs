use crate::Result;
use rand::SeedableRng;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};

/// Dataset row with optional transcriptions for each side.
///
/// `x_1` and `x_2` are the raw forms used for storage/export, while
/// `x_transcription` and `y_transcription` are used at scoring time by
/// algorithms that require phonetic input (for example, ALINE).
///
/// Deserialization behavior:
/// - `x` and `y` are required.
/// - `label`, `x_transcription`, and `y_transcription` are optional and default to `None`
///   when the corresponding CSV column is missing.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Row {
    #[serde(alias = "x", alias = "x_1")]
    pub x_1: String,
    #[serde(alias = "y", alias = "x_2")]
    pub x_2: String,
    #[serde(default)]
    pub label: Option<f32>,
    #[serde(default)]
    pub t_1: Option<String>,
    #[serde(default)]
    pub t_2: Option<String>,
}

/// Fluent builder for `Row` to enable ergonomic chaining of optional fields.
pub struct RowBuilder {
    x_1: String,
    x_2: String,
    label: Option<f32>,
    t_1: Option<String>,
    t_2: Option<String>,
}

impl Row {
    pub const COL_X_1: &'static str = "x_1";
    pub const COL_X_2: &'static str = "x_2";
    pub const COL_LABEL: &'static str = "label";
    pub const COL_T_1: &'static str = "t_1";
    pub const COL_T_2: &'static str = "t_2";

    /// Create a `Row` with the required fields `x` and `y`.
    ///
    /// Optional fields (`label`, `t_1`, `t_2`) can be
    /// added with the fluent builder: `Row::builder(x, y).label(...).build()`.
    pub fn new<S1, S2>(x: S1, y: S2) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        Self {
            x_1: x.into(),
            x_2: y.into(),
            label: None,
            t_1: None,
            t_2: None,
        }
    }

    /// Start a fluent builder for `Row`.
    ///
    /// Example: `Row::builder("a", "b").label(0.5).transcriptions("x", "y").build()`
    pub fn builder<S1, S2>(x: S1, y: S2) -> RowBuilder
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        RowBuilder {
            x_1: x.into(),
            x_2: y.into(),
            label: None,
            t_1: None,
            t_2: None,
        }
    }
}

impl RowBuilder {
    /// Set the optional label (target) for the row.
    pub fn label(mut self, value: f32) -> Self {
        self.label = Some(value);
        self
    }

    /// Set transcriptions for x and y.
    pub fn transcriptions<T1, T2>(mut self, x_tr: T1, y_tr: T2) -> Self
    where
        T1: Into<String>,
        T2: Into<String>,
    {
        self.t_1 = Some(x_tr.into());
        self.t_2 = Some(y_tr.into());
        self
    }

    /// Build the final `Row` value.
    pub fn build(self) -> Row {
        Row {
            x_1: self.x_1,
            x_2: self.x_2,
            label: self.label,
            t_1: self.t_1,
            t_2: self.t_2,
        }
    }
}

impl From<RowBuilder> for Row {
    fn from(b: RowBuilder) -> Self {
        b.build()
    }
}
impl Row {
    pub(super) fn pair_for<'a>(
        &'a self,
        requires_transcription: bool,
        algorithm_name: &str,
        row_index: usize,
    ) -> Result<(&'a str, &'a str)> {
        if !requires_transcription {
            return Ok((self.x_1.as_str(), self.x_2.as_str()));
        }

        match (self.t_1.as_deref(), self.t_2.as_deref()) {
            (Some(x_tr), Some(y_tr)) => Ok((x_tr, y_tr)),
            _ => Err(crate::Error::MissingTranscription {
                algorithm: algorithm_name.to_string(),
                row_index,
            }),
        }
    }
}

pub struct SplitConfig {
    /// Fraction of data to use for training (e.g. 0.8 for 80%).
    pub train_fraction: f64,
    /// If true, split is stratified by binarized label (label >= 0.5 => positive).
    pub stratify: bool,
    /// If true, balance classes in the training set by undersampling the majority class.
    pub balance: bool,
    /// Optional seed for reproducibility.
    pub seed: Option<u64>,
}

/// Split a slice of rows into (train, test) before precomputation.
///
/// This is the right place to split — rows are cheap, precomputed
/// score matrices are not. Call this first, then build a `ScoreMatrix`
/// from each half.
pub fn split_rows(rows: &[Row], config: &SplitConfig) -> Result<(Vec<Row>, Vec<Row>)> {
    if config.train_fraction <= 0.0 || config.train_fraction >= 1.0 {
        return Err(crate::Error::InvalidDatasetShape(
            "train_fraction must be in (0.0, 1.0)".to_string(),
        ));
    }

    let mut rng = match config.seed {
        Some(s) => StdRng::seed_from_u64(s),
        None => StdRng::from_entropy(),
    };

    // Partition indices into three strata: positive, negative, unlabeled.
    let (mut pos, mut neg, mut unl): (Vec<usize>, Vec<usize>, Vec<usize>) =
        (0..rows.len()).fold((vec![], vec![], vec![]), |(mut p, mut n, mut u), i| {
            match rows[i].label {
                Some(l) if l >= 0.5 => p.push(i),
                Some(_) => n.push(i),
                None => u.push(i),
            }
            (p, n, u)
        });

    pos.shuffle(&mut rng);
    neg.shuffle(&mut rng);
    unl.shuffle(&mut rng);

    let cut = |stratum: &[usize]| -> (Vec<usize>, Vec<usize>) {
        let n_train = ((stratum.len() as f64) * config.train_fraction).round() as usize;
        let n_train = n_train.max(1).min(stratum.len().saturating_sub(1));
        (stratum[..n_train].to_vec(), stratum[n_train..].to_vec())
    };

    let (train_idx, test_idx) = if config.stratify {
        let (tp, tep) = cut(&pos);
        let (tn, ten) = cut(&neg);
        let (tu, teu) = cut(&unl);

        let mut train = [tp, tn, tu].concat();
        let mut test = [tep, ten, teu].concat();
        train.shuffle(&mut rng);
        test.shuffle(&mut rng);
        (train, test)
    } else {
        let mut all: Vec<usize> = (0..rows.len()).collect();
        all.shuffle(&mut rng);
        let (tr, te) = cut(&all);
        (tr, te)
    };

    let train_idx = if config.balance {
        balance_indices(rows, train_idx, &mut rng)
    } else {
        train_idx
    };

    let train = train_idx.iter().map(|&i| rows[i].clone()).collect();
    let test = test_idx.iter().map(|&i| rows[i].clone()).collect();
    Ok((train, test))
}

fn balance_indices(rows: &[Row], indices: Vec<usize>, rng: &mut StdRng) -> Vec<usize> {
    let mut pos: Vec<_> = indices
        .iter()
        .copied()
        .filter(|&i| rows[i].label.map(|l| l >= 0.5).unwrap_or(false))
        .collect();
    let mut neg: Vec<_> = indices
        .iter()
        .copied()
        .filter(|&i| rows[i].label.map(|l| l < 0.5).unwrap_or(false))
        .collect();
    let unl: Vec<_> = indices
        .into_iter()
        .filter(|&i| rows[i].label.is_none())
        .collect();

    let minority = pos.len().min(neg.len());
    pos.shuffle(rng);
    pos.truncate(minority);
    neg.shuffle(rng);
    neg.truncate(minority);

    let mut out = [pos, neg, unl].concat();
    out.shuffle(rng);
    out
}
