use std::any::Any;
use std::collections::HashMap;
use std::fmt;
use uuid::Uuid;

// pub trait Component: Any + Send + Sync {}
pub trait Component: Any + Send + Sync + fmt::Debug {
}

// Storage for a specific component type
pub struct ComponentVec {
    // data: Vec<Option<Box<dyn Any>>>,
    pub(crate) data: HashMap<Uuid, Option<Box<dyn Any>>>,
}

impl ComponentVec {
    pub(crate) fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    // fn ensure_capacity(&mut self, entity_id: usize) {
    //     if entity_id >= self.data.len() {
    //         self.data.resize_with(entity_id + 1, || None);
    //     }
    // }

    pub(crate) fn insert(&mut self, entity_id: Uuid, component: Box<dyn Any>) {
        self.data.insert(entity_id, Some(component));
    }

    pub(crate) fn get(&self, entity_id: Uuid) -> Option<&dyn Any> {
        self.data.get(&entity_id)?.as_ref().map(|b| b.as_ref())
    }

    pub(crate) fn get_mut(&mut self, entity_id: Uuid) -> Option<&mut dyn Any> {
        self.data.get_mut(&entity_id)?.as_mut().map(|b| b.as_mut())
    }

    pub(crate) fn remove(&mut self, entity_id: Uuid) {
        self.data.remove(&entity_id);
    }
}
