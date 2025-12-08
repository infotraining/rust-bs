use std::fs::File;
use json_data::person::Person;

use json_data::person::create_people;

#[derive(Debug, thiserror::Error)]
enum JsonIoError {
    #[error("IO Error: {0}")]
    Io(#[from]std::io::Error),

    #[error("Serde JSON Error: {0}")]
    Json(#[from]serde_json::Error)
}

fn save_to_json_file(people: &Vec<Person>, path: &str) -> Result<(), JsonIoError> {
    // Serialize the vector of Person structs to JSON and save to a file
    let file = File::create(path)?;
    serde_json::to_writer_pretty(file, people)?;
    Ok(())
}

fn load_from_json_file(path: &str) -> Result<Vec<Person>, JsonIoError> {
    let file = File::open(path)?;
    let people: Vec<Person> = serde_json::from_reader(file)?;
    Ok(people)
}

fn main() {
    let people = create_people();
    
    let file_path = "people.json";
    match save_to_json_file(&people, file_path) {
        Ok(_) => println!("Save completed!!!"),
        Err(e) => println!("Something got wrong... {e}")
    };

    let loaded_people = load_from_json_file(file_path);
    match loaded_people {
        Ok(people) => {
            for person in people {
                println!("{:?}", person);
            };
        },
        Err(e) => { println!("Something got wrong... {e}"); }
    };
}