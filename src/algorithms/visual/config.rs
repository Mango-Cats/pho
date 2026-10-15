use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    Error, Result,
    utils::validate::{non_negative, validated_serde},
};

/// Edit costs for characters in the same group (`same`) and in different
/// groups (`diff`). Both must be finite and non-negative, with
/// `same <= diff` (identical characters always cost 0).
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(remote = "Self")]
pub struct Costs {
    pub(crate) same: f32,
    pub(crate) diff: f32,
}

validated_serde!(Costs);

impl Costs {
    pub fn validate(&self) -> Result<()> {
        non_negative("costs.same", self.same)?;
        non_negative("costs.diff", self.diff)?;
        if self.same > self.diff {
            return Err(Error::InvalidCostOrder {
                same: self.same,
                diff: self.diff,
            });
        }
        Ok(())
    }

    pub fn try_new(same: f32, diff: f32) -> Result<Self> {
        let costs = Self { same, diff };
        costs.validate()?;
        Ok(costs)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VisualWeighted {
    pub(crate) costs: Costs,
    pub(crate) group: HashMap<char, Vec<usize>>,
    #[serde(default)]
    pub(crate) case_insensitive: bool,
    #[serde(default)]
    pub(crate) separate: bool,
}

impl VisualWeighted {
    pub fn new(
        costs: Costs,
        group: HashMap<char, Vec<usize>>,
        case_insensitive: bool,
        separate: bool,
    ) -> Self {
        Self {
            costs,
            group,
            case_insensitive,
            separate,
        }
    }

    /// Sets the edit costs.
    pub fn set_costs(&mut self, costs: Costs) {
        self.costs = costs;
    }

    /// Sets the visual group table.
    pub fn set_group(&mut self, group: HashMap<char, Vec<usize>>) {
        self.group = group;
    }

    /// Sets whether to perform case-insensitive comparison.
    pub fn set_case_insensitive(&mut self, case_insensitive: bool) {
        self.case_insensitive = case_insensitive;
    }

    /// Sets whether to report separated edit-operation counts.
    pub fn set_separate(&mut self, separate: bool) {
        self.separate = separate;
    }
}
