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
  rigidbody::Rigidbody,
  physics::PhysicsSystem,
  transform::Transform,
  system::SystemManager,
  component::ComponentManager,
};

fn main() {
  // initialize systems
  let mut component_manager = ComponentManager::new();
  let mut system_manager = SystemManager::new();

  component_manager.register_components::<Transform>();
  component_manager.register_components::<Rigidbody>();

  system_manager.register_system::<PhysicsSystem>();

  // create three entities
  for _ in 0..3 {
    system_manager.entity_system.create_entity();
  }

  // create transform and rigidbody components for each entity
  for entity in 0..system_manager.entity_system.entities {
    let transform = Transform::default();
    let rigidbody = Rigidbody { vel: Vector2::new(1.0 * (entity as f64), 0.5 * (entity as f64)) };

    // physics_system.transform_manager.add_component(entity, Box::new(transform));
    // physics_system.rigidbody_manager.add_component(entity, Box::new(rigidbody));
  }

  system_manager.update(&mut component_manager);
  // physics_system.update(system_manager.entity_system.entities);
  // physics_system.transform_manager.print_transforms();
}
