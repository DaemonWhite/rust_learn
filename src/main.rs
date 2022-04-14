//Include stdio
use std::io::{self, Write};

mod rand_number_game;

fn help() {
    println!("\ntaper g pour lancer le jeux trouver un nombre aléatoire");
    println!("taper f pour fermer le programe");
    println!("taper h pour l'aide");
}

fn main() {

    println!("Bienvenue dans mon programe rust");
    println!("v 0.0.1a");
    help();

    loop {
        let mut choice= String::new();

        print!("\nVotre choix : ");

        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut choice)
            .expect("Echec de la lecture de l'entrée utilisateur");

        let choice: &str = &*choice;

        match  choice {
            "g\n" => rand_number_game::games(),
            "h\n" => help(),
            "f\n" => break,
            _=> print!("choix non existant taper h pour l'aide")
        }
    }

    println!("Fermeture de l'application");
}
