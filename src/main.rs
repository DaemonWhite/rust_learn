//Include stdio
mod ihm;

fn main() {
    ihm::start();

    loop {
        let choix = ihm::switch();

        if choix == "f" {
            break;
        }
    }
}
