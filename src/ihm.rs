use std::io::{self, Write};
use std::cmp::Ordering;

mod utilities;

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
fn termui_games(){
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

pub fn start(){
    println!("Bienvenue dans mon programe rust");
    println!("v 0.0.1a");
    help();
}

pub fn switch(opt: String) -> String {
    let mut choice = opt;

    if "" == choice {
        print!("\nmenu -> Votre choix : ");

        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut choice)
            .expect("Echec de la lecture de l'entrée utilisateur");
    }

    let choice: &str = &*choice.trim();

    match  choice {
        "g" => termui_games(),
        "c" => extend_string(),
        "h" => help(),
        "f" => println!("Fermeture de l'application"),
        _=> println!("choix non existant taper h pour l'aide")
    }

    choice.to_string()
}