use crate::pet::Pet;

pub struct PetStore {
    pets: Vec<Box<dyn Pet>>,
}

impl PetStore {
    pub fn new() -> Self {
        PetStore { pets: Vec::new() }
    }

    // Add a pet using generics
    pub fn add_pet<T: Pet + 'static>(&mut self, pet: T) {
        self.pets.push(Box::new(pet));
    }

    pub fn list_pets(&self) {
        if self.pets.is_empty() {
            println!("No pets in store!");
            return;
        }

        println!("\nPet Store Inventory:");
        for (i, pet) in self.pets.iter().enumerate() {
            println!("{}. {}", i + 1, pet.get_info());
            println!("   Sound: {}", pet.make_sound());
        }
    }

    pub fn find_pet(&self, name: &str) -> Option<&Box<dyn Pet>> {
        self.pets.iter().find(|pet| pet.get_info().contains(name))
    }
}