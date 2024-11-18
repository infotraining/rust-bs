use std::collections::{BTreeMap, HashMap};
use std::io::Read;
use regex::Regex;

fn tokenize(text: &str) -> Vec<&str> {
    //let re = Regex::new(r"\w+|\b\w+'\w+\b").unwrap();
    let re = Regex::new(r"\w+").unwrap();

    re.find_iter(text).map(|m| m.as_str()).collect()
}

fn concordance(words: Vec<&str>) -> HashMap<&str, u32> {
    let mut concordance = HashMap::new();

    for word in words {
        let entry = concordance.entry(word).or_insert(0);
        *entry += 1;
    }

    concordance
}

fn main() {
    let mut file = std::fs::File::open("holmes.txt").expect("Could not open file");

    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();

    let words = tokenize(&text);
    println!("Number of tokens: {}", words.len());

    let counted_words = concordance(words).iter().map(|(&word, &count)| (count, word)).collect::<BTreeMap<_, _>>();

    for (count, word) in counted_words.iter().rev().take(10) {
        println!("{}: {}", word, count);
    }
}
