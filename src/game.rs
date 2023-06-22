use crate::{dice::Dice, player::Player};
use io::stdin;
use std::{cell::RefCell, io};

static DICE_ROLLS: u8 = 2;

pub struct DiceGame {
    players: Vec<Player>,
}

impl DiceGame {
    pub fn new(players: Vec<Player>) -> Self {
        Self { players }
    }

    pub fn play(&self) -> DiceGameMatch {
        let mut the_match = DiceGameMatch::from(self);
        the_match.run();
        the_match
    }
}

pub struct DiceGameMatch {
    players: RefCell<Vec<Player>>,
    winner: Option<Player>,
}

impl From<&DiceGame> for DiceGameMatch {
    fn from(value: &DiceGame) -> Self {
        DiceGameMatch {
            players: RefCell::new(value.players.clone()),
            winner: None,
        }
    }
}

impl DiceGameMatch {
    pub fn winner(&self) -> Option<&Player> {
        self.winner.as_ref()
    }

    pub fn is_over(&self) -> bool {
        self.players.borrow().len() == 1
    }

    /// Removes a player from this game
    fn remove_player(&self, who: &Player) {
        let maybe_idx;
        {
            maybe_idx = self.players.borrow().iter().position(|p| p == who);
        }

        if let Some(idx) = maybe_idx {
            self.players.borrow_mut().remove(idx);
        }
    }

    /// This just prints the current state of the game
    fn print_game_info(&self) {
        println!(
            "I giocatori in gara sono: {}",
            self.players
                .borrow()
                .iter()
                .map(|p| p.name())
                .collect::<Vec<&str>>()
                .join(", ")
        );
    }

    pub fn run(&mut self) {
        let mut i: usize = 0;

        while !self.is_over() {
            self.print_game_info();
            let mut players_count = self.players.borrow().len();
            let mut players = self.players.borrow_mut();
            if let Some(current_player) = players.get_mut(i) {
                while current_player.has_money() {
                    println!(
                        "Giocatore {}, inserisci la tua puntata, valore portafogli {}",
                        current_player.name(),
                        current_player.wallet()
                    );

                    let bet: i128 = read_bet();

                    if bet <= current_player.wallet() {
                        *current_player.wallet_mut() -= bet;

                        let totale_dadi = roll_dices(DICE_ROLLS);
                        match totale_dadi {
                            n if n >= 8 => *current_player.wallet_mut() += bet * 2,
                            _ => {}
                        }
                    }

                    match current_player.wallet() {
                        n if n <= 0 => {
                            println!(
                                "il giocatore {} ha terminato i soldi. Ha perso.",
                                current_player.name()
                            );
                            self.remove_player(current_player);
                            players_count -= 1;
                        }
                        _ => {}
                    }

                    i = match i == players_count - 1 {
                        true => 0,
                        false => i + 1,
                    };
                }
            }
        }

        let mut remaining_player = self.players.borrow_mut();
        self.winner = Some(self.players.borrow_mut().iter().next().unwrap().clone());
        remaining_player.clear();
    }
}

fn read_bet() -> i128 {
    let mut input = String::from("");
    stdin()
        .read_line(&mut input)
        .expect("Errore nella lettura dell'input");

    let puntata: i128 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Riprova ad inserire il valore della puntata");
            read_bet()
        }
    };
    puntata
}

fn roll_dices(n_times: u8) -> u8 {
    let mut total: u8 = 0;
    let dice = Dice(6);
    for i in 0..n_times {
        let roll_result = dice.roll();
        println!("Lancio dado numero: {i}, esce il numero: {roll_result}");
        total += roll_result;
    }
    println!("Il totale uscito dai dadi e': {total}");
    total
}
