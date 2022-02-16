mod system;
mod component;
mod entity;
mod vector2;
mod transform;
mod rigidbody;
mod gravity;
mod physics;

use crate::{
  vector2::Vector2,
  physics::PhysicsSystem,
  transform::Transform,
  rigidbody::Rigidbody,
  gravity::Gravity,
  system::SystemManager,
  component::ComponentManager,
};

fn main() {
  // initialize managers
  let mut component_manager = ComponentManager::new();
  let mut system_manager = SystemManager::new();

  // register systems
  system_manager.register_system::<PhysicsSystem>();

  // create three entities
  for _ in 0..3 {
    system_manager.entity_system.create_entity();
  }

  // create transform and rigidbody components for each entity
  for entity in 0..system_manager.entity_system.entities {
    let transform = Transform::default();
    let rigidbody = Rigidbody { vel: Vector2::new(1.0 * (entity as f64), 0.5 * (entity as f64)) };
    let gravity = Gravity(9.8);

    component_manager.add_component::<Transform>(entity, transform);
    component_manager.add_component::<Rigidbody>(entity, rigidbody);
    component_manager.add_component::<Gravity>(entity, gravity);
  }

  // run update a few times and print new values
  for _ in 0..3 {
    system_manager.update(&mut component_manager);
    if let Some(c) = component_manager.get_components::<Transform>() {
      c.print_all_components();
    }
  }
}
