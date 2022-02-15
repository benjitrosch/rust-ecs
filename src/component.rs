use std::{
    collections::HashMap,
    any::{
        TypeId,
        Any
    }
};
  
pub trait Component {}

pub struct ComponentList {
    pub transform_components: Vec<Box<dyn Component>>,
    pub entity_map: HashMap<usize, usize>
}

impl ComponentList {
    fn add_component(&mut self, entity: usize, component: Box<dyn Component>) {
        let index = self.transform_components.len();
    
        self.entity_map.insert(entity, index);
        self.transform_components.push(component);
    }

    fn get_entity_component(&mut self, entity: usize) -> &mut Box<dyn Component> {
        let index = self.entity_map.get(&entity).unwrap();
        &mut self.transform_components[*index]
    }
}

pub struct ComponentManager {
    pub components: HashMap<TypeId, Box<dyn Component>>
}

impl ComponentManager {  
    pub fn get_component<T: 'static>(&self) -> &Box<dyn Component> where T : Component {
        &self.components.get(&TypeId::of::<T>()).unwrap()
    }
}
