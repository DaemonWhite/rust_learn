use std::env;

mod ihm;

fn switcher_argument(opt: &str) -> bool {

    let mut exit: &str = "f";
    let mut ret:bool = false;

    match  opt {
        "-g" | "--games" => exit="g",
        "-c" | "--compare" => exit="c",
        "-h" | "--help" => ihm::help_argument(),
        _ => {
            ihm::error_argumet();
            ret= true;
        }
    }

    if !ret {
        ihm::switch(exit);
    }

    ret
}

fn argument() -> bool {

    let args: Vec<String> = env::args().collect();
    let size_args = args.len();
    let mut close: bool = false;

    for i in 1..size_args {
        let data_args: &str = &args[i];
        close = switcher_argument( data_args );
        if close {
            break;
        }
    }

    close
}

fn main() {
    let no_start: bool = argument();


    if !no_start {
        ihm::start();

        loop {
            let choix = ihm::switch("");

            if choix == "f" {
            break;
            }
        }
    }
}
