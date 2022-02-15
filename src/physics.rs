use crate::{
    system::System,
    manager::Manager,
    transform::TransformManager,
    rigidbody::RigidbodyManager,
    vector2::Vector2
};

pub struct PhysicsSystem {
    pub transform_manager: TransformManager,
    pub rigidbody_manager: RigidbodyManager
}

impl System for PhysicsSystem {
    fn update(&mut self, entities: usize) {
        for entity in 0..entities {
            let has_transform_component = self.transform_manager.entity_map.contains_key(&entity);
            let has_rigidbody_component = self.rigidbody_manager.entity_map.contains_key(&entity);

            if has_transform_component && has_rigidbody_component {
                let transform = self.transform_manager.get_entity_component(entity);
                let rigidbody = self.rigidbody_manager.get_entity_component(entity);

                transform.pos = Vector2::new(
                    transform.pos.x + rigidbody.vel.x,
                    transform.pos.y + rigidbody.vel.y
                );
            }
        }
    }
}