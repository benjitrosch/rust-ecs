use crate::component::Component;

pub trait Manager {
  type T: Component;
  fn add_component(&mut self, entity: usize, component: Box<Self::T>);
  fn get_entity_component(&mut self, entity: usize) -> &mut Self::T;
}
