// Main entry point for the pet store program
// This program demonstrates a pet store system, creating pets with different types
// (dog, cat, bird), adding them to a store, listing them, and searching by name.
//Max Quirk and Josh Schmidt

// Declare modules used in the program (assumed to be in separate files)
mod pet;        // Contains Pet and PetInfo definitions
mod pet_store;  // Contains PetStore definition and functionality
mod pet_types;  // Contains PetType enum and additional data structs (DogData, CatData, BirdData)

// Import specific items from the modules for use in main
use pet::{Pet, PetInfo};                    // Pet trait and PetInfo struct
use pet_store::PetStore;                    // PetStore struct for managing pets
use pet_types::{BirdData, CatData, DogData, PetType}; // Pet type enum and data structs

// Main function: entry point of the program
fn main() {
    // Create a new, empty PetStore instance to manage pets
    let mut store = PetStore::new();

    // Create a dog pet with specific attributes
    let dog = PetInfo {
        name: "Max".to_string(),                 // Name of the pet
        age: 3,                                  // Age in years
        pet_type: PetType::Dog,                  // Type of pet (Dog variant)
        price: 250.00,                           // Price in dollars
        additional: DogData {                    // Dog-specific data
            breed: "Golden Retriever".to_string(), // Breed of the dog
            is_trained: true,                    // Whether the dog is trained
        },
    };

    // Create a cat pet with specific attributes
    let cat = PetInfo {
        name: "Luna".to_string(),                // Name of the pet
        age: 2,                                  // Age in years
        pet_type: PetType::Cat,                  // Type of pet (Cat variant)
        price: 150.00,                           // Price in dollars
        additional: CatData {                    // Cat-specific data
            is_litter_trained: true,             // Whether the cat is litter-trained
        },
    };

    // Create a bird pet with specific attributes
    let bird = PetInfo {
        name: "Polly".to_string(),               // Name of the pet
        age: 1,                                  // Age in years
        pet_type: PetType::Bird,                 // Type of pet (Bird variant)
        price: 75.00,                            // Price in dollars
        additional: BirdData {                   // Bird-specific data
            can_talk: true,                      // Whether the bird can talk
        },
    };

    // Add the created pets to the pet store
    store.add_pet(dog);   // Add the dog (Max)
    store.add_pet(cat);   // Add the cat (Luna)
    store.add_pet(bird);  // Add the bird (Polly)

    // Display all pets currently in the store
    store.list_pets();

    // Demonstrate searching for a pet by name
    println!("\nSearching for Max:");
    // Attempt to find a pet named "Max" in the store
    if let Some(pet) = store.find_pet("Max") {
        // If found, print the pet’s information using its get_info method
        println!("Found: {}", pet.get_info());
    } else {
        // If not found, print an error message
        println!("Pet not found!");
    }
}