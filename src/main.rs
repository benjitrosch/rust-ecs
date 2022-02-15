mod system;
mod manager;
mod component;
mod entity;
mod vector2;
mod transform;
mod rigidbody;
mod physics;

use crate::{
  vector2::Vector2,
  system::System,
  system::SystemManager,
  manager::Manager,
  transform::Transform,
  rigidbody::Rigidbody,
  transform::TransformManager,
  rigidbody::RigidbodyManager,
  physics::PhysicsSystem,
};

fn main() {
  // initialize systems
  let transform_manager = TransformManager::new();
  let rigidbody_manager = RigidbodyManager::new();

  let mut system_manager = SystemManager::new();

  let mut physics_system = PhysicsSystem {
    transform_manager,
    rigidbody_manager
  };

  system_manager.register_system(Box::new(physics_system));

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

  system_manager.update();
  // physics_system.update(system_manager.entity_system.entities);
  // physics_system.transform_manager.print_transforms();
}
