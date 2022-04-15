use std::io::{self, Write};
use std::cmp::Ordering;

mod rand_number_game;

fn termui_games(){
    let num_search: u32 = rand_number_game::rand_number(1, 101);
    println!("Trouver le bon nombre");

    loop {
        print!("Veuillez entrer un nombre  : ");

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

        //switch avec comparateur cmp Ordering
        match supposition.cmp(&num_search) {
            Ordering::Less => println!("C'est plus!"),
            Ordering::Greater => println!("C'est moins"),
            Ordering::Equal => {
                println!("Vous avez gagné");
                break;
            }
        }
    }
}

fn help() {
    println!("\ntaper g pour lancer le jeux trouver un nombre aléatoire");
    println!("taper f pour fermer le programe");
    println!("taper h pour l'aide");
}

pub fn start(){
    println!("Bienvenue dans mon programe rust");
    println!("v 0.0.1a");
    help();
}

pub fn switch() -> String {
    let mut choice= String::new();

    print!("\nmenu -> Votre choix : ");

    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut choice)
        .expect("Echec de la lecture de l'entrée utilisateur");

    let choice: &str = &*choice.trim();

    match  choice {
        "g" => termui_games(),
        "h" => help(),
        "f" => println!("Fermeture de l'application"),
        _=> println!("choix non existant taper h pour l'aide")
    }

    return choice.to_string();
}
