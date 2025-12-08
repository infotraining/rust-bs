use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
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