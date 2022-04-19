use std::env;

mod ihm;

fn argument() {

    let args: Vec<String> = env::args().collect();
    let size_args = args.len();

    for i in 1..size_args {
        let data_args: &String = &args[i];
        ihm::switch( data_args.to_string() );
    }

}

fn main() {

    argument();
    ihm::start();

    loop {
        let choix = ihm::switch("".to_string());

        if choix == "f" {
            break;
        }
    }
}
