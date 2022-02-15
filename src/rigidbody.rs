use std::collections::HashMap;
use crate::vector2::Vector2;
use crate::manager::Manager;
use crate::component::Component;

pub struct Rigidbody {
    pub vel: Vector2
}

impl Component for Rigidbody {}

pub struct RigidbodyManager {
    pub rigidbody_components: Vec<Rigidbody>,
    pub entity_map: HashMap<usize, usize>
}
  
impl RigidbodyManager {
    pub fn new() -> Self {
        Self {
            rigidbody_components: Vec::new(),
            entity_map: HashMap::new()
        }
    }
}
  
impl Manager for RigidbodyManager {
    type T = Rigidbody;
    
    fn add_component(&mut self, entity: usize, component: Box<Self::T>) {
      let index = self.rigidbody_components.len();
  
      self.entity_map.insert(entity, index);
      self.rigidbody_components.push(*component);
    }

    fn get_entity_component(&mut self, entity: usize) -> &mut Self::T {
        let index = self.entity_map.get(&entity).unwrap();
        &mut self.rigidbody_components[*index]
    }
}
  