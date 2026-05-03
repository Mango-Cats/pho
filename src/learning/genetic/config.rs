// src/learning/genetic/types.rs

/// Configuration for the genetic algorithm optimizer.
#[derive(Debug, Clone)]
pub struct GeneticConfig {
    /// Total number of individuals in the population.
    pub population_size: usize,
    /// Number of generations to run the evolution.
    pub generations: usize,
    /// Probability (0.0–1.0) of a weight undergoing mutation each generation.
    pub mutation_rate: f32,
    /// Maximum magnitude of a single mutation adjustment.
    pub mutation_step: f32,
    /// Number of candidates sampled per tournament during selection.
    pub tournament_size: usize,
    /// Number of top individuals carried over unchanged to the next generation (elitism).
    pub elite_count: usize,
}

impl GeneticConfig {
    pub fn validate(&self) -> Result<(), crate::error::Error> {
        if self.population_size == 0 {
            return Err(crate::error::Error::InvalidFeatureSum {
                feature: "population_size",
                sum: 0.0,
            });
        }

        if self.generations == 0 {
            return Err(crate::error::Error::InvalidFeatureSum {
                feature: "generations",
                sum: 0.0,
            });
        }

        if !(0.0..=1.0).contains(&self.mutation_rate) {
            return Err(crate::error::Error::InvalidFeatureSum {
                feature: "mutation_rate",
                sum: self.mutation_rate,
            });
        }

        if self.mutation_step < 0.0 {
            return Err(crate::error::Error::NegativeEpsilon(self.mutation_step));
        }

        if self.tournament_size == 0 {
            return Err(crate::error::Error::InvalidFeatureSum {
                feature: "tournament_size",
                sum: 0.0,
            });
        }

        if self.elite_count > self.population_size {
            return Err(crate::error::Error::InvalidFeatureSum {
                feature: "elite_count",
                sum: self.elite_count as f32,
            });
        }

        Ok(())
    }

    pub fn try_new(
        population_size: usize,
        generations: usize,
        mutation_rate: f32,
        mutation_step: f32,
        tournament_size: usize,
        elite_count: usize,
    ) -> Result<Self, crate::error::Error> {
        let config = Self {
            population_size,
            generations,
            mutation_rate,
            mutation_step,
            tournament_size,
            elite_count,
        };

        config.validate()?;
        Ok(config)
    }
}
