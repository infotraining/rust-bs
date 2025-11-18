use std::error::Error;
use std::fs::File;
use json_data::person::Person;

use json_data::person::create_people;

#[derive(Debug, thiserror::Error)]
enum JsonIOError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serde JSON error: {0}")]
    Serde(#[from] serde_json::Error)
}

fn save_to_json_file(people: &Vec<Person>, path: &str) -> Result<(), JsonIOError> {
    let json_file = File::create(path)?;
    serde_json::to_writer_pretty(json_file, people)?;
    Ok(())
}

fn load_from_json_file(path: &str) -> Result<Vec<Person>, JsonIOError> {
    let json_file = File::open(path)?;
    let people: Vec<Person> = serde_json::from_reader(json_file)?;
    Ok(people)
}

fn main() {
    let people = create_people();
    save_to_json_file(&people, "people.json").expect("Failed to save json file!");


    let loaded_people =
        match load_from_json_file("people.json") {
            Err(e) => {
                eprintln!("Error loading JSON file: {}", e);
                return;
            },
            Ok(people) => people,
        };
    
    for person in &loaded_people {
        println!("{:?}", person);
    }
}
