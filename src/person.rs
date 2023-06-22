/// This represents a single player
#[derive(Debug)]
pub struct Player {
    name: String,
    #[allow(dead_code)]
    age: u32,
    wallet: i128
}

impl Player {
    /// Crates a new player with a `name`, an `age` and an initial `wallet`
    pub fn new(name: &str, age: u32, wallet: i128) -> Self {
        Self {
            name: String::from(name),
            age,
            wallet
        }
    }

    /// Returns the name of the player
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Returns the age of the player
    #[allow(dead_code)]
    pub fn age(&self) -> u32 {
        self.age
    }

    /// Returns the wallet balance for `self`
    pub fn wallet(&self) -> i128 {
        self.wallet
    }

    /// Returns a mutable reference to the wallet balance for `self`
    pub fn waller_mut(&mut self) -> &mut i128 {
        &mut self.wallet
    }

}