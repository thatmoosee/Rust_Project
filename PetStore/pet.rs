use std::fmt;

use crate::pet_types::PetType;

// Define a trait for common pet behavior
pub trait Pet {
    fn get_info(&self) -> String;
    fn make_sound(&self) -> String;
}

// Define pet struct with generic type for additional data
#[derive(Debug)]
pub struct PetInfo<T> {
    pub name: String,
    pub age: u8,
    pub pet_type: PetType,
    pub price: f32,
    pub additional: T,
}

// Implement Pet trait for PetInfo
impl<T: fmt::Debug> Pet for PetInfo<T> {
    fn get_info(&self) -> String {
        format!(
            "Name: {}, Age: {}, Type: {:?}, Price: ${:.2}",
            self.name, self.age, self.pet_type, self.price
        )
    }

    fn make_sound(&self) -> String {
        match self.pet_type {
            PetType::Dog => "Woof!".to_string(),
            PetType::Cat => "Meow!".to_string(),
            PetType::Bird => "Chirp!".to_string(),
        }
    }
}