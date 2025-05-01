//Different pet types and stuff
//Max Quirk and Josh Schmidt

// Import the std::fmt module for formatting traits (used by Debug derivations)
use std::fmt;

// PetType enum: represents the possible types of pets in the pet store
#[derive(Debug)]
pub enum PetType {
    Dog,   // Represents a dog
    Cat,   // Represents a cat
    Bird,  // Represents a bird
}

// DogData struct: stores additional data specific to dogs
#[derive(Debug)]
pub struct DogData {
    pub breed: String,       // The breed of the dog (e.g., "Golden Retriever")
    pub is_trained: bool,    // Whether the dog is trained
}

// CatData struct: stores additional data specific to cats
#[derive(Debug)]
pub struct CatData {
    pub is_litter_trained: bool, // Whether the cat is litter-trained
}

// BirdData struct: stores additional data specific to birds
#[derive(Debug)]
pub struct BirdData {
    pub can_talk: bool,      // Whether the bird can talk
}