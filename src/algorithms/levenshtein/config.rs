use serde::{Deserialize, Serialize};

use crate::{
    Result,
    utils::validate::{non_negative, validated_serde},
};

/// Cost configuration for Levenshtein distance operations.
///
/// These costs determine the penalty for each edit operation when
/// computing the distance between two strings.
/// All costs must be finite and non-negative.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(remote = "Self")]
pub struct Costs {
    /// Cost for inserting a character.
    pub(crate) insert: f32,
    /// Cost for deleting a character.
    pub(crate) delete: f32,
    /// Cost for substituting one character with another.
    pub(crate) substitute: f32,
}

validated_serde!(Costs);

impl Costs {
    pub fn validate(&self) -> Result<()> {
        non_negative("costs.insert", self.insert)?;
        non_negative("costs.delete", self.delete)?;
        non_negative("costs.substitute", self.substitute)
    }

    pub fn try_new(insert: f32, delete: f32, substitute: f32) -> Result<Self> {
        let costs = Self {
            insert,
            delete,
            substitute,
        };
        costs.validate()?;
        Ok(costs)
    }
}

/// Configuration for the Levenshtein distance algorithm.
///
/// This structure holds the cost parameters that control how
/// different edit operations are weighted when computing string distance.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Levenshtein {
    pub(crate) costs: Costs,
    pub(crate) case_insensitive: bool,
    #[serde(default)]
    pub(crate) separate: bool,
    #[serde(default)]
    pub(crate) consonants_only: bool,
}

impl Levenshtein {
    pub fn new(costs: Costs, case_insensitive: bool) -> Self {
        Self {
            costs,
            case_insensitive,
            separate: false,
            consonants_only: false,
        }
    }

    /// Sets the edit costs.
    pub fn set_costs(&mut self, costs: Costs) {
        self.costs = costs;
    }

    /// Sets whether to perform case-insensitive comparison.
    pub fn set_case_insensitive(&mut self, case_insensitive: bool) {
        self.case_insensitive = case_insensitive;
    }

    /// Sets whether to report separated edit-operation counts.
    pub fn set_separate(&mut self, separate: bool) {
        self.separate = separate;
    }

    /// Sets whether to compare consonants only.
    pub fn set_consonants_only(&mut self, consonants_only: bool) {
        self.consonants_only = consonants_only;
    }
}
