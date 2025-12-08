use json_data::person::Person;
use std::fs::File;

use json_data::person::create_people;

fn save_to_json_file(people: &Vec<Person>, path: &str) {
    // Serialize the vector of Person structs to JSON and save to a file
    let file = File::create(path).unwrap();
    serde_json::to_writer_pretty(file, people).unwrap();
}

fn load_from_json_file(path: &str) -> Vec<Person> {
    let file = File::open(path).unwrap();
    let people: Vec<Person> = serde_json::from_reader(file).unwrap();
    people
}

fn main() {
    let people = create_people();

    let file_path = "people.json";
    save_to_json_file(&people, file_path);

    let loaded_people = load_from_json_file(file_path);

    for person in loaded_people {
        println!("{:?}", person);
    }
}
