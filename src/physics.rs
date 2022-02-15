use crate::{
    system::System,
    vector2::Vector2,
    component::ComponentManager,
    transform::Transform,
    rigidbody::Rigidbody
};

pub struct PhysicsSystem {}

impl Default for PhysicsSystem {
    fn default() -> Self {
        Self {}    
    }  
}

impl System for PhysicsSystem {
    fn update(&mut self, entities: usize, component_manager: &mut ComponentManager) {
        let mut transform_list = component_manager.get_components::<Transform>();
        // let rigidbody_list = component_manager.get_components::<Rigidbody>();

        for entity in 0..entities {
            let has_transform_component = transform_list.entity_map.contains_key(&entity);
            // let has_rigidbody_component = rigidbody_list.entity_map.contains_key(&entity);

            if has_transform_component {//&& has_rigidbody_component {
                let transform = transform_list.get_entity_component(entity);
                // let rigidbody = rigidbody_list.get_entity_component(entity);

                // transform.pos = Vector2::new(
                //     transform.pos.x + rigidbody.vel.x,
                //     transform.pos.y + rigidbody.vel.y
                // );
            }
        }
    }
}