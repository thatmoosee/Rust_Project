//Pet Store implementation
//By Max Quirk and Josh Schmidt

// Import the Pet trait from the pet module, assuming it’s defined in pet.rs
use crate::pet::Pet;

// PetStore struct: manages a collection of pets in the pet store
pub struct PetStore {
    pets: Vec<Box<dyn Pet>>, // Vector of boxed pets, using dynamic dispatch for Pet trait objects
}

// Implementation of PetStore methods
impl PetStore {
    // Creates a new, empty PetStore instance
    pub fn new() -> Self {
        PetStore {
            pets: Vec::new(), // Initialize an empty vector to store pets
        }
    }

    // Adds a pet to the store using generics
    // T must implement the Pet trait and have a 'static lifetime (for Box<dyn Pet>)
    pub fn add_pet<T: Pet + 'static>(&mut self, pet: T) {
        // Box the pet to store it as a trait object and push it to the pets vector
        self.pets.push(Box::new(pet));
    }

    // Lists all pets in the store, displaying their information and sounds
    pub fn list_pets(&self) {
        // Check if the store is empty
        if self.pets.is_empty() {
            println!("No pets in store!");
            return;
        }

        // Print a header for the inventory list
        println!("\nPet Store Inventory:");
        // Iterate over the pets with enumeration to display index numbers
        for (i, pet) in self.pets.iter().enumerate() {
            // Print the pet’s index (1-based) and its info (from get_info)
            println!("{}. {}", i + 1, pet.get_info());
            // Print the sound the pet makes (from make_sound)
            println!("   Sound: {}", pet.make_sound());
        }
    }

    // Finds a pet by name, returning a reference to its