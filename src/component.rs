use std::{
    collections::HashMap,
    any::{
        TypeId,
        Any
    }
};
  
pub trait Component {}

pub struct ComponentList {
    pub components: Vec<Box<dyn Component>>,
    pub entity_map: HashMap<usize, usize>
}

impl ComponentList {
    pub fn new() -> Self {
        Self {
            components: Vec::new(),
            entity_map: HashMap::new()
        }
    }

    pub fn add_component(&mut self, entity: usize, component: Box<dyn Component>) {
        let index = self.components.len();
    
        self.entity_map.insert(entity, index);
        self.components.push(component);
    }

    pub fn get_entity_component(&self, entity: usize) -> &Box<dyn Component> {
        let index = self.entity_map.get(&entity).unwrap();
        &self.components[*index]
    }
}

pub struct ComponentManager {
    pub component_lists: HashMap<TypeId, ComponentList>
}

impl ComponentManager {  
    pub fn new() -> Self {
        Self {
            component_lists: HashMap::new()
        }
    }

    pub fn register_components<T: 'static>(&mut self) where T : Component {
        self.component_lists.insert(TypeId::of::<T>(), ComponentList::new());
    }

    pub fn get_components<T: 'static>(&mut self) -> &ComponentList where T : Component {
        &self.component_lists.get(&TypeId::of::<T>()).unwrap()
    }

    // pub fn get_entity_component<T: 'static>(&mut self, entity: usize) -> &Box<dyn Component> where T : Component {
    //     let components = self.get_components::<T>();
    //     components.get_entity_component(entity)
    // }

    pub fn add_component(&mut self, entity: usize, component: Box<dyn Component>) {
        let type_id = component.type_id();
        let mut component_lists = self.component_lists.get(&type_id).unwrap();
        // components.add_component(entity, component);
    }
}
