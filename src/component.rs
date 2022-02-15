use std::{
    collections::HashMap,
    any::{
        TypeId,
        Any
    },
};
  
pub trait Component {}

pub struct ComponentList<T> {
    pub components: Vec<T>,
    pub entity_map: HashMap<usize, usize>
}

impl<T: Component> ComponentList<T> {
    pub fn new() -> Self {
        Self {
            components: Vec::<T>::new(),
            entity_map: HashMap::new()
        }
    }

    pub fn add_component(&mut self, entity: usize, component: T) {
        let index = self.components.len();
    
        self.entity_map.insert(entity, index);
        self.components.push(component);
    }

    pub fn get_entity_component(&self, entity: usize) -> &T {
        let index = self.entity_map.get(&entity).unwrap();
        &self.components[*index]
    }

    pub fn get_entity_component_mut(&mut self, entity: usize) -> &mut T {
        let index = self.entity_map.get(&entity).unwrap();
        &mut self.components[*index]
    }
}

pub struct ComponentManager {
    pub component_lists: HashMap<TypeId, Box<dyn Any>>
}

impl ComponentManager {  
    pub fn new() -> Self {
        Self {
            component_lists: HashMap::new()
        }
    }

    pub fn register_components<T: 'static>(&mut self) where T : Component {
        self.component_lists.insert(TypeId::of::<T>(), Box::new(ComponentList::<T>::new()));
    }

    pub fn get_components<T: 'static>(&self) -> &ComponentList<T> where T : Component {
        let boxed_component_lists = self.component_lists.get(&TypeId::of::<T>()).unwrap();
        boxed_component_lists.downcast_ref::<ComponentList<T>>().unwrap()
    }

    pub fn get_components_mut<T: 'static>(&mut self) -> &mut ComponentList<T> where T : Component {
        let boxed_component_lists = self.component_lists.get_mut(&TypeId::of::<T>()).unwrap();
        boxed_component_lists.downcast_mut::<ComponentList<T>>().unwrap()
    }

    pub fn get_entity_component<T: 'static>(&self, entity: usize) -> &T where T : Component {
        let components = self.get_components::<T>();
        components.get_entity_component(entity)
    }

    pub fn get_entity_component_mut<T: 'static>(&mut self, entity: usize) -> &mut T where T : Component {
        let components = self.get_components_mut::<T>();
        components.get_entity_component_mut(entity)
    }

    pub fn add_component<T: 'static>(&mut self, entity: usize, component: T) where T : Component {
        let component_lists = self.get_components_mut::<T>();
        component_lists.add_component(entity, component);
    }

    pub fn entity_has_component<T: 'static>(&self, entity: usize) -> bool where T : Component {
        let component_lists = self.get_components::<T>();
        component_lists.entity_map.contains_key(&entity)
    }
}
