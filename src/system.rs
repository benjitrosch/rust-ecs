use std::{
  collections::HashMap,
  any::{
      TypeId,
      Any
  }
};

use crate::{
  entity::EntitySystem
};

pub trait System {
  fn update(&mut self, entities: usize);
}

pub struct SystemManager {
  pub entity_system: EntitySystem,
  pub systems: HashMap<TypeId, Box<dyn System>>
}

impl SystemManager {
  pub fn new() -> Self {
      Self {
          entity_system: EntitySystem::new(),
          systems: HashMap::new(),
      }
  }

  pub fn register_system(&mut self, system: Box<dyn System>) {
      self.systems.insert(system.type_id(), system);
  }

  pub fn get_system<T: 'static>(&self) -> &Box<dyn System> where T : System {
      &self.systems.get(&TypeId::of::<T>()).unwrap()
  }

  pub fn update(&mut self) {
      for (_, system) in &mut self.systems {
          system.update(self.entity_system.entities);
      }
  }
}
