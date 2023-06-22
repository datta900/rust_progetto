use rand::{Rng, thread_rng};

/// A dice with a given number of faces
pub struct Dice(pub u8);

impl Dice {
    /// Rolls the dice
    pub fn roll(&self) -> u8 {
        thread_rng().gen_range(1u8..=self.0)
    }
}
