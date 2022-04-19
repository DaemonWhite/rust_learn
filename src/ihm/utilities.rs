use rand::Rng;

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
