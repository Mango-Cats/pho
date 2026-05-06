use super::feature_types::{Airstream, Binary, Manner, Phonation, Place, SecondaryArticulation};
use super::phoneme_types::CommonFeatures;

/// Shared interface for accessing common features.
pub trait Phoneme {
    fn place(&self) -> &Place;
    fn manner(&self) -> &Manner;
    fn syllabic(&self) -> &Binary;
    fn voice(&self) -> &Binary;
    fn nasal(&self) -> &Binary;
    fn retroflex(&self) -> &Binary;
    fn lateral(&self) -> &Binary;
    fn phonation(&self) -> &Phonation;
    fn airstream(&self) -> &Airstream;
    fn secondary(&self) -> &SecondaryArticulation;
}

impl Phoneme for CommonFeatures {
    fn place(&self) -> &Place {
        &self.place
    }
    fn manner(&self) -> &Manner {
        &self.manner
    }
    fn syllabic(&self) -> &Binary {
        &self.syllabic
    }
    fn voice(&self) -> &Binary {
        &self.voice
    }
    fn nasal(&self) -> &Binary {
        &self.nasal
    }
    fn retroflex(&self) -> &Binary {
        &self.retroflex
    }
    fn lateral(&self) -> &Binary {
        &self.lateral
    }

    fn phonation(&self) -> &Phonation {
        &self.phonation
    }
    fn airstream(&self) -> &Airstream {
        &self.airstream
    }
    fn secondary(&self) -> &SecondaryArticulation {
        &self.secondary
    }
}
