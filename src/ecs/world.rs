use std::{any::Any, collections::HashMap};

use super::systems::*;

pub struct ECSWorld {
    entities: HashMap<u32, HashMap<String, Box<dyn Any>>>, // Map entity IDs to components
    systems: Vec<Box<dyn System>>, // Vector of systems
}

impl ECSWorld {
    pub fn new() -> Self {
        ECSWorld {
            entities: HashMap::new(),
            systems: vec![],
        }
    }

    pub fn add_entity(&mut self, entity_id: u32, components: HashMap<String, Box<dyn Any>>) {
        self.entities.insert(entity_id, components);
    }

    pub fn add_system(&mut self, system: Box<dyn System>) {
        self.systems.push(system);
    }

    pub fn update(&mut self) {
        for system in &mut self.systems {
            system.update(&mut self.entities);
        }
    }
}
