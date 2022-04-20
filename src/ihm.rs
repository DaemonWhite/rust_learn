use std::io::{self, Write};
use std::cmp::Ordering;

mod utilities;

use crate::lib::Word;


fn extend_string()
{
    let mut text_a = String::new();
    let mut text_b = String::new();
    let mut tampon = String::new();

    print!("Entrer votre texte : ");
    io::stdout().flush().unwrap();

     io::stdin()
            .read_line(&mut text_a)
            .expect("Echec de la lecture de l'entrée utilisateur");

    print!("Entrer l'extention du texte : ");
    io::stdout().flush().unwrap();

    io::stdin()
            .read_line(&mut tampon)
            .expect("Echec de la lecture de l'entrée utilisateur");

    text_b += text_a.trim();
    text_b.push_str(" ");
    text_b += tampon.trim();

    let (text_a, size_a) = utilities::size_string(text_a.trim().to_string());
    let (text_b, size_b) = utilities::size_string(text_b);
    let diff = utilities::simpl_less(size_b, size_a);

    println!("taille du texte : {}. Est de : {}", text_a.trim(), size_a);
    println!("A pour diférences {} - {} = {} du texte: {}", size_b, size_a, diff, text_b.trim());

}

fn games_solve_word() {
    let mut word = Word::new();

    println!("Donner le mot a trouver");

    io::stdin()
            .read_line(&mut word.search)
            .expect("Echec de la lecture de l'entrée utilisateur");

    word.search = word.search.trim().to_string();

    utilities::clear();

    word.mixt = utilities::simple_melangeur(&word.search);

    word.add();

    println!("ok : {} \n {}", word.mixt, word.same());

}


fn games_solve_number(){
    let num_search: u32 = utilities::rand_number(1, 101);
    println!("Trouver le bon nombre");

    loop {
        let mut supposition = String::new();

        print!("Veuillez entrer un nombre  : ");
        io::stdout().flush().unwrap();

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
    println!("taper c pour soustraire deux texte");
    println!("taper h pour l'aide");
}

pub fn version() {

    let machine_kind = if cfg!(target_os = "linux") {
            "linux"
        } else if cfg!(target_os = "windows") {
            "windows"
        } else {
            "unknown"
    };

    println!("{}: 0.0.1a\n", machine_kind)
}

pub fn error_argumet() {

    eprintln!("Erreur --> argument inconue");

}

pub fn help_argument() {

    version();

    println!("-g | --games\t lance le jeux
-c | --compare\t lance la comparaison
-h | --help\t voire l'aide
    ");

}

pub fn start(){
    println!("Bienvenue dans mon programe rust");
    version();
    help();
}

pub fn switch(opt: &str) -> String {
    let mut choice: String = opt.to_string();

    if "" == choice {
        print!("menu -> Votre choix : ");

        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut choice)
            .expect("Echec de la lecture de l'entrée utilisateur");
    }

    let choice: &str = &*choice.trim();

    match  choice {
        "g" => games_solve_number(),
        "w" => games_solve_word(),
        "c" => extend_string(),
        "h" => help(),
        "f" => println!("Fermeture de l'application"),
        _=> println!("choix non existant taper h pour l'aide")
    }

    choice.to_string()
}
