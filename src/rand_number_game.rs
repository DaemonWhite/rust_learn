use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Write};

pub fn games() {
    println!("teste de la mort !");

    let nombre_secret = rand::thread_rng().gen_range(1..101);

    println!("Le nombre secret est : {}", nombre_secret);

    loop {
        print!("Veuillez entrer un nombre : ");

        io::stdout().flush().unwrap();

        let mut supposition = String::new();

        io::stdin()
            .read_line(&mut supposition)
            .expect("Echec de la lecture de l'entrée utilisateur");

        println!("Votre nombre : {}", supposition);

        let supposition: u32 = match supposition.trim().parse() {
            Ok(nombre) => nombre,
            Err(_) => continue,
        };

        match supposition.cmp(&nombre_secret) {
        Ordering::Less => println!("C'est plus!"),
        Ordering::Greater => println!("C'est moins"),
        Ordering::Equal => {
            println!("Vous avez gagné");
            break;
            }
        }
    }
}
