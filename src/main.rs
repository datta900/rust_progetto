//lo scopo del progetto è iniziare a muovere i primi passi verso Rust
//Inizialmente, con un portafoglio di 50€, si potranno effettuare due lanci,
//se la somma dei dati è inferiore a 8 si perdono 25, altrimenti guadagni 15

use rand::Rng;
fn main() {
    loop{
        let mut portafoglio : i32 = 50; //saldo iniziale
        let numero_dadi : i8 = 2;

        let totale_dadi = lancio_dadi(numero_dadi);

        if(totale_dadi < 8){
            portafoglio -= 15;
        }else{
            portafoglio += 15;
        }
    }
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
