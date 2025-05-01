use std::fmt;

#[derive(Debug)]
pub enum PetType {
    Dog,
    Cat,
    Bird,
}

#[derive(Debug)]
pub struct DogData {
    pub breed: String,
    pub is_trained: bool,
}

#[derive(Debug)]
pub struct CatData {
    pub is_litter_trained: bool,
}

#[derive(Debug)]
pub struct BirdData {
    pub can_talk: bool,
}