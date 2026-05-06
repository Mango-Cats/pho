use crate::algorithms::Algorithm;

type ScoreFn = dyn Fn(&str, &str) -> crate::Result<f32> + 'static;

pub struct WeightedFunction {
    name: String,
    requires_transcription: bool,
    pub weight: f32,
    score_fn: Box<ScoreFn>,
}

impl WeightedFunction {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn requires_transcription(&self) -> bool {
        self.requires_transcription
    }

    pub fn score(&self, x: &str, y: &str) -> crate::Result<f32> {
        (self.score_fn)(x, y)
    }

    pub fn from_function<N, F>(
        name: N,
        weight: f32,
        requires_transcription: bool,
        score_fn: F,
    ) -> Self
    where
        N: Into<String>,
        F: Fn(&str, &str) -> crate::Result<f32> + 'static,
    {
        Self {
            name: name.into(),
            requires_transcription,
            weight,
            score_fn: Box::new(score_fn),
        }
    }

    fn from_algorithm_method<A, M>(algorithm: A, weight: f32, method: M) -> Self
    where
        A: Algorithm + 'static,
        M: Fn(&A, &str, &str) -> crate::Result<f32> + 'static,
    {
        let name = algorithm.name().to_string();
        let requires_transcription = algorithm.requires_transcription();

        Self::from_function(name, weight, requires_transcription, move |x, y| {
            method(&algorithm, x, y)
        })
    }

    pub fn from_similarity<A: Algorithm + 'static>(alg: A, weight: f32) -> Self {
        Self::from_algorithm_method(alg, weight, A::similarity)
    }

    pub fn from_normalized_distance<A: Algorithm + 'static>(alg: A, weight: f32) -> Self {
        Self::from_algorithm_method(alg, weight, A::normalized_distance)
    }

    pub fn from_distance<A: Algorithm + 'static>(alg: A, weight: f32) -> Self {
        Self::from_algorithm_method(alg, weight, A::distance)
    }
}
