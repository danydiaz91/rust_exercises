use std::env;
use std::io;
use std::fs;

fn main() {
    
}

pub struct Config {
    filename: String
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next(); // Escape first argument

        match args.next() {
            Some(filename) => Ok(Config {filename}),
            None => Err("Didn't get a file name")
        }
    }
}

pub fn read_file(config: Config) -> io::Result<Vec<String>> {
    let contents = fs::read_to_string(config.filename)?;
    let words: Vec<String> = contents.split(' ')
        .map(|word| word.trim().to_string()) // Maybe is not the best way?
        .collect();

    Ok(words)
}

#[cfg(test)]
pub mod tests {
    use super::*;


    #[test]
    fn read_file_ok() {
        let config = Config {filename: String::from("file_example.txt")};
        let words = read_file(config).expect("Something went wrong reading the file");
        let expect: Vec<String> = vec!("Rust:", "safe,", "fast,", "productive.", "Pick", "three.")
            .iter()
            .map(|word| word.to_string())
            .collect();

        assert_eq!(expect, words);
    }

    #[test]
    fn read_file_error() {
        let config = Config {filename: String::from("file_example_error.txt")};
        assert!(read_file(config).is_err());
    }
}
