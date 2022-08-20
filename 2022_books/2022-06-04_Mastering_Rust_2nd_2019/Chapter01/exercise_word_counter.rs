// Chapter 01 Exercise

// A program that counts instances of words
// in a text file given to it as its argument

use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{prelude::BufRead, BufReader},
};

#[derive(Debug)]
struct WordCounter(HashMap<String, u64>);

impl WordCounter {
    fn new() -> WordCounter {
        WordCounter(HashMap::new())
    }

    fn increment(&mut self, word: &str) {
        let key = word.to_string();
        let count = self.0.entry(key).or_insert(0);
        *count += 1;
    }

    fn display(self) {
        for (key, value) in self.0.iter() {
            println!("{}: {}", key, value);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("Processing file: {}", filename);

    let file = File::open(filename).expect("Could not open file");
    let reader = BufReader::new(file);

    let mut word_counter = WordCounter::new();

    for line in reader.lines() {
        let line = line.expect("Could not read line");

        for word in line.split(" ") {
            if word != "" {
                word_counter.increment(word);
            }
        }
    }

    word_counter.display();
}
