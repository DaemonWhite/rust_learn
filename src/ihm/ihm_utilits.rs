use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, WriteColor, StandardStream};

pub fn info_col(adv:u8, text: &String) {

    let mut entete = String::new();
    let mut color: &str = "white";
    let error: String = "Erreur".to_string();
    let adver: String = "Avertisement".to_string();
    let info: String = "Info".to_string();

    match adv {
        3 => {
            entete = error;
            color = "red";
        },
        2 => {
            entete = adver;
            color = "yellow";
        },
        1 => {
            entete = info;
            color = "blue";
        },
        0 => entete = "".to_string(),
        _ => eprint!("{} Valeur incorecte", adv),
    }
    print!("[");
    colors(color, entete);
    print!("]");
    println!(" {}", text);
}



pub fn colors(color: &str, s: String)  {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Blue))).expect("impossible de definir la couleur");

    match color {
        "red" => stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red))).expect("impossible de definir la couleur"),
        "yellow" => stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow))).expect("impossible de definir la couleur"),
        "blue" => stdout.set_color(ColorSpec::new().set_fg(Some(Color::Blue))).expect("impossible de definir la couleur"),
        "green" => stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green))).expect("impossible de definir la couleur"),
        _ => eprint!("{} Valeur incorecte", color),
    }

    write!(&mut stdout, "{}", s).expect("Erreur l'ors de l'écriture");

    stdout.set_color(ColorSpec::new().set_fg(Some(Color::White))).expect("impossible de definir la couleur");
    write!(&mut stdout, "").expect("Erreur l'ors de l'écriture");
}


