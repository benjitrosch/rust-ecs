use std::collections::HashMap;
use crate::vector2::Vector2;
use crate::manager::Manager;
use crate::component::Component;

pub struct Transform {
  pub pos: Vector2
}

impl Default for Transform {
  fn default() -> Self {
    Self {
      pos: Vector2::new(0.0, 0.0)
    }    
  }  
}

impl Component for Transform {}

pub struct TransformManager {
  pub transform_components: Vec<Transform>,
  pub entity_map: HashMap<usize, usize>
}

impl TransformManager {
  pub fn new() -> Self {
    Self {
      transform_components: Vec::new(),
      entity_map: HashMap::new()
    }
  }

  pub fn print_transforms(&self) {
    for (entity, index) in &self.entity_map {
      let transform = &self.transform_components[*index];
      println!("entity: {}, pos: ({}, {})", entity, transform.pos.x, transform.pos.y);
    }
  }
}

impl Manager for TransformManager {
  type T = Transform;
  
  fn add_component(&mut self, entity: usize, component: Box<Self::T>) {
    let index = self.transform_components.len();

    self.entity_map.insert(entity, index);
    self.transform_components.push(*component);
  }

  fn get_entity_component(&mut self, entity: usize) -> &mut Self::T {
    let index = self.entity_map.get(&entity).unwrap();
    &mut self.transform_components[*index]
  }
}
