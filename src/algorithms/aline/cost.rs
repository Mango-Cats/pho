/// This struct holds the cost constants for the Aline algorithm.
///
/// ## Cost Variables
///
/// Aline uses four constants for reward or penalty; a negative value
/// denotes a penalty while a positive denotes a reward.
///
/// 1. `skip` is the constant for an indel (insert or delete).
///
/// 2. `subtitute` is the constant for a substitution (when one phoneme
/// is replaced with another).
///
/// 3. `expand_compress` is the constant for when a phoneme matches two
/// phonemes in another. Example: "suit" can be pronounced as [sut] and
/// [suwt], so the /u/ sound is expanded to the /uw/ sound.
///
/// 4. `vowel_consonant` is the relative weight for vowels versus
/// consonants.
///
/// ## References
///
/// - https://dl.acm.org/doi/book/10.5555/936774
pub struct Costs {
    pub skip: i32,
    pub substitute: i32,
    pub expand_compress: i32,
    pub vowel_consonant: i32,
}

/// This constant stores the default costs from Kondrak's PhD Dissertation,
/// but we also use the `vowel_consonant` value from Natural Language Toolkit
/// which is 5 instead of 10.
///
/// ## Usage
///
/// ```rust
/// use pho::algorithms::aline::cost::Costs;
/// let costs = Costs::default();
///
/// assert!(*(&costs.skip) == -10, "The default value for skip is not -10!");
/// assert!(*(&costs.substitute) == 35, "The default value for substitute is not 35!");
/// ```
///
/// ## References
///
/// - https://www.nltk.org/
impl Default for Costs {
    fn default() -> Self {
        Self {
            skip: -10,
            substitute: 35,
            expand_compress: 45,
            vowel_consonant: 5,
        }
    }
}
