use rand::Rng;

pub fn rand_number(min: u32, max: u32) -> u32 {
    //Nombre aléatoire de 1 à 100
    let nombre_secret = rand::thread_rng().gen_range(min..max);
    nombre_secret
}
