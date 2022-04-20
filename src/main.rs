use std::env;

mod ihm;
mod lib;

fn switcher_argument(opt: &str) -> bool {

    let mut exit: &str = "f";
    let mut ret:bool = false;

    match  opt {
        "-n" | "--number" => exit="n",
        "-s" | "--search" => exit="s",
        "-c" | "--compare" => exit="c",
        "-v" | "--version" => {
            ihm::version();
            ret=true;
        },
        "-h" | "--help" => {
            ihm::help_argument();
            ret=true;
        },
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
