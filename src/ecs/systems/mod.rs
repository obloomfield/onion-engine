pub mod movement_system;

use std::{any::Any, collections::HashMap};

pub trait System {
  fn update(&mut self, entities: &mut HashMap<u32, HashMap<String, Box<dyn Any>>>);
}

