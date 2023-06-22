mod dice;
mod game;
mod player;

use game::DiceGame;
use player::Player;

fn main() {
    let dario = Player::new("Dario", 29, 120);
    let nicholas = Player::new("Nicholas", 27, 120);
    let chiara = Player::new("Chiara", 25, 120);

    //    let mut persone_giocanti: [Persona; 2] = [dario, nicholas, chiara];
    let players = vec![dario, nicholas, chiara];
    let game = DiceGame::new(players);

    if let Some(winner) = game.play().winner() {
        println!(
            "Il vincitore è: {} con un portafogli di {}",
            winner.name(),
            winner.wallet()
        );
    }
}
