mod person;
mod dice;

use io::stdin;
use person::Player;
use std::io;
use dice::Dice;

fn main() {
    let dario = Player::new("Dario", 29, 120);
    let nicholas = Player::new("Nicholas", 27, 120);
    let chiara = Player::new("Chiara", 25, 120);

    let numero_dadi: u8 = 2;

    //    let mut persone_giocanti: [Persona; 2] = [dario, nicholas, chiara];
    let mut persone_giocanti = vec![dario, nicholas, chiara];
    let mut i: usize = 0;
 
    while persone_giocanti[i].wallet() > 0 && persone_giocanti.len() > 1 {
        leggi_elementi_inizio_gara(&persone_giocanti);
 
        println!(
            "Giocatore {}, inserisci la tua puntata, valore portafogli {}",
            persone_giocanti[i].name(),
            persone_giocanti[i].wallet()
        );
    
        let puntata: i128 = inserimento_puntata();

        if puntata <= persone_giocanti[i].wallet() {
            *persone_giocanti[i].waller_mut() -= puntata;

            let totale_dadi = roll_dices(numero_dadi);
            match totale_dadi {
                n @ _ if n >= 8 => *persone_giocanti[i].waller_mut() += puntata * 2,
                _ => {}
            }
        }

        match persone_giocanti[i].wallet() {
            n @ _ if n <= 0 => {
                println!(
                    "il giocatore {} ha terminato i soldi. Ha perso.",
                    persone_giocanti[i].name()
                );
                persone_giocanti.remove(i);
            }
            _ => {}
        }

        i += 1;

        if persone_giocanti.len() == i || persone_giocanti.len() < i {
            i = 0;
        }
    }

    println!(
        "Il vincitore è: {} con un portafogli di {}",
        persone_giocanti[0].name(),
        persone_giocanti[0].wallet()
    );
}

fn inserimento_puntata() -> i128 {
    let mut input = String::from("");
    stdin()
        .read_line(&mut input)
        .expect("Errore nella lettura dell'input");

    let puntata: i128 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Riprova ad inserire il valore della puntata");
            inserimento_puntata()
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

fn leggi_elementi_inizio_gara(persone_giocanti: &[Player]) {
    let mut giocatori_in_gara = String::from("I giocatori in gara sono: ");
    for persona in persone_giocanti {
        giocatori_in_gara.push_str(&persona.name());
        giocatori_in_gara.push_str(" ");
    }
    println!("{}", giocatori_in_gara);
}
