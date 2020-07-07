use exercise_2 as lib;
use std::env;
use std::process;

fn main() {
    let config = lib::Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let words = lib::read_file(config).unwrap_or_else(|err| {
        eprintln!("Problem reading file: {}", err.to_string());
        process::exit(1);
    });

    let unique_words = exercise_2::into_hashset(words);

    lib::print_if_exists(&unique_words, vec!["Rust:", "fast,"]);
    lib::print_if_exists(&unique_words, vec!["speed", "Pick", "hola"]);
    lib::print_if_exists(&unique_words, vec!["ru1st", "not", "bi"]);
}
