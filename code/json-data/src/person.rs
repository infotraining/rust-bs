use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Id(pub u32);

impl Id {
    pub fn value(&self) -> u32 {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Address {
    pub street: String,
    pub city: String,
    pub country: String,
}

impl Address {
    pub fn new(street: &str, city: &str, country: &str) -> Self {
        Address {
            street: street.to_string(),
            city: city.to_string(),
            country: country.to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Person {
    pub id: Id,
    pub name: String,
    pub age: u32,
    pub address: Option<Address>,
}

impl Person {
    pub fn new(id: Id, name: &str, age: u32) -> Self {
        Person {
            id,
            name: name.to_string(),
            age,
            address: None,
        }
    }
}

pub struct PersonBuilder {
    id: Id,
    name: String,
    age: u32,
    address: Option<Address>,
}

impl PersonBuilder {
    pub fn new(id: Id, name: &str, age: u32) -> Self {
        PersonBuilder {
            id,
            name: name.to_string(),
            age,
            address: None,
        }
    }

    pub fn address(mut self, street: &str, city: &str, country: &str) -> Self {
        self.address = Some(Address::new(street, city, country));
        self
    }

    pub fn build(self) -> Person {
        Person {
            id: self.id,
            name: self.name,
            age: self.age,
            address: self.address,
        }
    }
}

pub fn create_people() -> Vec<Person> {
    vec![
        PersonBuilder::new(Id(1), "Alice", 30)
            .address("123 Main St", "Wonderland", "Fictionland")
            .build(),
        PersonBuilder::new(Id(2), "Bob", 35)
            .address("456 Side St", "Wonderland", "Fictionland")
            .build(),
        PersonBuilder::new(Id(3), "Charlie", 28).build(),
    ]
}

#[cfg(test)]
mod person_tests {
    use super::*;

    #[test]
    fn create_people_returns_vec_of_person() {
        let people = create_people();

        assert_eq!(people.len(), 3);
        assert_eq!(people[0].name, "Alice");
        assert_eq!(people[1].address.as_ref().unwrap().city, "Wonderland");
        assert!(people[2].address.is_none());
    }
}