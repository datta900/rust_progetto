//lo scopo del progetto è iniziare a muovere i primi passi verso Rust
//Inizialmente, con un portafoglio di 50€, si potranno effettuare due lanci,
//se la somma dei dati è inferiore a 8 si perdono 25, altrimenti guadagni 15

use std::io;
use rand::Rng;
use io::stdin;

#[derive(Debug)]
struct Persona {
    nome: String,
    eta: u32,
    portafogli : i128
}

fn main() {

    let dario = Persona {
        nome:String::from("Dario"),
        eta: 26,
        portafogli:50
    };

    let nicholas = Persona{
        nome: String::from("Nicholas"),
        eta:27,
        ..dario
    };

    let chiara = Persona{
        nome: String::from("Chiara"),
        eta:25,
        ..dario
    };


    let numero_dadi : i8 = 2;

//    let mut persone_giocanti: [Persona; 2] = [dario, nicholas, chiara];
    let mut persone_giocanti = vec![dario, nicholas, chiara];
    let mut i : usize = 0;
    while persone_giocanti[i].portafogli > 0 && persone_giocanti.len() > 1{
        leggi_elementi_inizio_gara(&persone_giocanti);
        println!("Giocatore {}, inserisci la tua puntata, valore portafogli {}", persone_giocanti[i].nome, persone_giocanti[i].portafogli);
        let mut puntata : i128 = inserimento_puntata();

        if puntata <= persone_giocanti[i].portafogli {
            persone_giocanti[i].portafogli -= puntata;


            let totale_dadi = lancio_dadi(numero_dadi);
            match totale_dadi {
                n @ _ if n >= 8 => persone_giocanti[i].portafogli += (puntata*2),
                _ =>  {},
            }
        }

        match persone_giocanti[i].portafogli{
            n @ _ if n <= 0 => {
                println!("il giocatore {} ha terminato i soldi. Ha perso.", persone_giocanti[i].nome);
                persone_giocanti.remove(i);
            }
            _ => {}
        }

        i+=1;

        if persone_giocanti.len() == i || persone_giocanti.len() < i{
            i = 0;
        }

    }

    println!("Il vincitore è: {} con un portafogli di {}", persone_giocanti[0].nome, persone_giocanti[0].portafogli);
}

fn inserimento_puntata() -> i128{
    let mut input= String::from("");
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

fn lancio_dadi(dice_number : i8) -> i8{
    let mut totale_dadi : i8 = 0;
    for i in 0..dice_number {
        let numero_lancio_dado = rand::thread_rng().gen_range(1..=6);
        println!("Lancio dado numero: {i}, esce il numero: {numero_lancio_dado}");
        totale_dadi += numero_lancio_dado;
    }
    println!("Il totale uscito dai dadi e': {totale_dadi}");
    return totale_dadi;
}

fn leggi_elementi_inizio_gara(persone_giocanti : &[Persona]){
    let mut giocatori_in_gara = String::from("I giocatori in gara sono: ");
    for persona in persone_giocanti{
        giocatori_in_gara.push_str(&persona.nome);
        giocatori_in_gara.push_str(" ");
    }
    println!("{}", giocatori_in_gara);
}
