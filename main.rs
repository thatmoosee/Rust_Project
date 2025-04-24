mod pet;
mod pet_store;
mod pet_types;

use pet::{Pet, PetInfo};
use pet_store::PetStore;
use pet_types::{BirdData, CatData, DogData, PetType};

fn main() {
    let mut store = PetStore::new();

    // Create some pets with different additional data
    let dog = PetInfo {
        name: "Max".to_string(),
        age: 3,
        pet_type: PetType::Dog,
        price: 250.00,
        additional: DogData {
            breed: "Golden Retriever".to_string(),
            is_trained: true,
        },
    };

    let cat = PetInfo {
        name: "Luna".to_string(),
        age: 2,
        pet_type: PetType::Cat,
        price: 150.00,
        additional: CatData {
            is_litter_trained: true,
        },
    };

    let bird = PetInfo {
        name: "Polly".to_string(),
        age: 1,
        pet_type: PetType::Bird,
        price: 75.00,
        additional: BirdData { can_talk: true },
    };

    // Add pets to store
    store.add_pet(dog);
    store.add_pet(cat);
    store.add_pet(bird);

    // List all pets
    store.list_pets();

    // Demonstrate finding a pet
    println!("\nSearching for Max:");
    if let Some(pet) = store.find_pet("Max") {
        println!("Found: {}", pet.get_info());
    } else {
        println!("Pet not found!");
    }
}