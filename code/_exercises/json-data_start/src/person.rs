// TODO - implement reqiured trats
use serde::{Deserialize, Serialize};
use crate::address::Address;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Id(pub u32);

impl Id {
    pub fn value(&self) -> u32 {
        self.0
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
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
    fn id_test() {
        let id = Id(42);
        assert_eq!(id.value(), 42);
    }

    #[test]
    fn person_creation() {
        let person = PersonBuilder::new(Id(1), "Test User", 25)
            .address("789 Test St", "Testville", "Testland")
            .build();

        assert_eq!(person.id.value(), 1);
        assert_eq!(person.name, "Test User");
        assert_eq!(person.age, 25);
        assert!(person.address.is_some());
        
        let address = person.address.unwrap();
        assert_eq!(address.street, "789 Test St");
        assert_eq!(address.city, "Testville");
        assert_eq!(address.country, "Testland");
    }

    #[test]
    fn person_creation_without_address() {
        let person = PersonBuilder::new(Id(2), "No Address", 40).build();
        assert_eq!(person.id.value(), 2);
        assert_eq!(person.name, "No Address");
        assert_eq!(person.age, 40);
        assert!(person.address.is_none());
    }
}