use rand::Rng;
use clearscreen::ClearScreen;

use crate::lib::VecteurString;

pub fn rand_number(min: u32, max: u32) -> u32 {
    //Nombre aléatoire de 1 à 100
    let nombre_secret = rand::thread_rng().gen_range(min..max);
    nombre_secret
}

pub fn size_string(s: String) -> (String, usize) {
    let size = s.len();

    (s, size)
}

pub fn simpl_less(a: usize,b: usize) -> usize {

    let ret;

    if a>=b {
        ret=a-b;
    } else {
        ret=b-a;
    }

    ret
}

pub fn clear() {

    if cfg!( target_os = "windows" ) {
        ClearScreen::Cls.clear().expect("Impossible d'éffacer le terminale");
    } else {
        ClearScreen::default().clear().expect("Impossible d'éffacer le terminal");
    }
}

pub fn simple_melangeur(s: &String) -> String {
    let mut text = VecteurString::new(&s);
    let mut ret = String::new();

    for _i in 0..text.len() {
        let nombre_secret: u32 = rand::thread_rng().gen_range(0..text.len());
        ret += &text.remove(nombre_secret);
    }

    ret
}
