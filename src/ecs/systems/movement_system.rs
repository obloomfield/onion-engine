use std::{any::Any, collections::HashMap};

use crate::ecs::components::{Position, Velocity};

use super::System;

pub struct MovementSystem;

impl MovementSystem {
    pub fn new() -> Self {
        MovementSystem
    }
}

impl System for MovementSystem {
  fn update(&mut self, entities: &mut HashMap<u32, HashMap<String, Box<dyn Any>>>) {
      // Create a clone of the entities hashmap to avoid borrow checker issues
      
      for (entity_id, components) in entities.iter_mut() {
          // Check if the entity has both Position and Velocity components
          if let (Some(position), Some(velocity)) = (
              components.get_mut("Position").and_then(|pos| pos.downcast_mut::<Position>()),
              components.get_mut("Velocity").and_then(|vel| vel.downcast_mut::<Velocity>()),
          ) {
              // Update the position based on velocity
              position.x += velocity.dx;
              position.y += velocity.dy;
          }
      }
  }
}
