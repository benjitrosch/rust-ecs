use crate::{
    system::System,
    vector2::Vector2,
    component::ComponentManager,
    transform::Transform,
    rigidbody::Rigidbody,
    gravity::Gravity,
};

pub struct PhysicsSystem {}

impl Default for PhysicsSystem {
    fn default() -> Self {
        Self {}    
    }  
}

impl System for PhysicsSystem {
    fn update(&mut self, entities: usize, component_manager: &mut ComponentManager) {
        for entity in 0..entities {
            let has_transform_component = component_manager.entity_has_component::<Transform>(entity);
            let has_rigidbody_component = component_manager.entity_has_component::<Rigidbody>(entity);
            let has_gravity_component = component_manager.entity_has_component::<Gravity>(entity);

            if has_transform_component && has_rigidbody_component && has_gravity_component {
                let transform_list = component_manager.get_components_mut::<Transform>();
                let rigidbody_list = component_manager.get_components_mut::<Rigidbody>();
                let gravity_list = component_manager.get_components::<Gravity>();

                let transform = transform_list.get_entity_component_mut(entity);
                let rigidbody = rigidbody_list.get_entity_component_mut(entity);
                let gravity = gravity_list.get_entity_component(entity);

                rigidbody.vel = Vector2::new(
                    rigidbody.vel.x,
                    rigidbody.vel.y - gravity.0,
                );

                transform.pos = Vector2::new(
                    transform.pos.x + rigidbody.vel.x,
                    transform.pos.y + rigidbody.vel.y
                );
            }
        }
    }
}