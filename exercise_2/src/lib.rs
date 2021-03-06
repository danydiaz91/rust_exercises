use std::collections::HashSet;
use std::env;
use std::fs;
use std::io;
use std::iter::FromIterator;
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

pub fn into_hashset(words: Vec<String>) -> HashSet<String> {
    HashSet::from_iter(words.into_iter()) // With iter doesn't work, why?
}

pub fn print_if_exists(unique_words: &HashSet<String>, words_to_print: Vec<&str>) {
    for word in words_to_print {
        let yes_no = if unique_words.contains(word) { "Yes" } else { "No" };
        println!("{}: {}", word, yes_no);
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn read_file_ok() {
        let config = Config {filename: String::from("file_example.txt")};
        let words = read_file(config).expect("Something went wrong reading the file");
        let expected: Vec<String> = vec!["Rust:", "safe,", "fast,", "productive.", "Pick", "three."]
            .iter()
            .map(|word| word.to_string())
            .collect();

        assert_eq!(expected, words);
    }

    #[test]
    fn read_file_error() {
        let config = Config {filename: String::from("file_example_error.txt")};
        assert!(read_file(config).is_err());
    }

    #[test]
    fn unique_words_ok() {
        let words: Vec<String> = vec!["Rust", "safe", "safe", "rust"]
            .iter()
            .map(|word| word.to_string())
            .collect();

        let actual = into_hashset(words);
        let mut expected: HashSet<String> = HashSet::new();
        expected.insert(String::from("Rust"));
        expected.insert(String::from("safe"));
        expected.insert(String::from("rust"));

        assert_eq!(expected, actual)
    }
}