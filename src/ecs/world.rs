use crate::creatures::components::Creature;
use crate::ecs::component::{Component, ComponentVec};
use crate::ecs::components::Position;
use crate::ecs::entity::Entity;
use crate::map::base_terrain::{Map, Terrain};
use crate::props::components::Prop;
use log::warn;
use std::any::{Any, TypeId};
use std::collections::HashMap;
use uuid::Uuid;

// The World holds all entities and components
pub struct World {
    // next_entity_id: u64,
    components: HashMap<TypeId, ComponentVec>,
    maps: HashMap<Uuid, Box<dyn Any>>,
    creature_lookup: HashMap<String, Uuid>,
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

impl World {
    pub fn new() -> Self {
        Self {
            // next_entity_id: 0,
            components: HashMap::new(),
            maps: HashMap::new(),
            creature_lookup: HashMap::new(),
        }
    }
    pub fn add_map<T: Terrain>(&mut self, map: Map<T>) {
        if self.maps.contains_key(&map.id) {
            warn!(
                "Map id {} already exists. WILL OVERWRITE WITH NEW MAP.",
                map.id
            );
        }
        self.maps.insert(map.id, Box::new(map));
    }

    // Create a new entity
    pub fn create_entity(&mut self) -> Entity {
        let new_id = Uuid::new_v4();
        Entity(new_id)
    }

    pub fn create_creature(&mut self, creature: Creature) -> Entity {
        // TODO: Apply the creature systems (health calculations, effect application, personality)
        let new_entity = self.create_entity();
        self.creature_lookup
            .insert(creature.name.clone(), new_entity.0);
        self.add_component(new_entity, creature);
        self.add_component(
            new_entity,
            Position {
                map: None,
                x: 0,
                y: 0,
            },
        );
        new_entity
    }

    pub fn create_prop(&mut self, prop_name: &str, prop_description: &str) -> Entity {
        let new_entity = self.create_entity();
        let new_prop = Prop {
            name: String::from(prop_name),
            description: String::from(prop_description),
        };
        self.add_component(new_entity, new_prop);
        new_entity
    }

    // Add a component to an entity
    pub fn add_component<T: Component>(&mut self, entity: Entity, component: T) {
        let type_id = TypeId::of::<T>();
        let storage = self
            .components
            .entry(type_id)
            .or_insert_with(ComponentVec::new);
        storage.insert(entity.0, Box::new(component));
    }

    // Get an immutable reference to a component
    pub fn get_component<T: Component>(&self, entity: Entity) -> Option<&T> {
        let type_id = TypeId::of::<T>();
        self.components
            .get(&type_id)?
            .get(entity.get_uuid())?
            .downcast_ref::<T>()
    }

    // Get a mutable reference to a component
    pub fn get_component_mut<T: Component>(&mut self, entity: Entity) -> Option<&mut T> {
        let type_id = TypeId::of::<T>();
        self.components
            .get_mut(&type_id)?
            .get_mut(entity.get_uuid())?
            .downcast_mut::<T>()
    }

    // Remove a component from an entity
    pub fn remove_component<T: Component>(&mut self, entity: Entity) {
        let type_id = TypeId::of::<T>();
        if let Some(storage) = self.components.get_mut(&type_id) {
            storage.remove(entity.get_uuid());
        }
    }

    pub fn get_creature_by_name(&self, name: &str) -> Option<(Entity, &Creature)> {
        match self.creature_lookup.get(name) {
            Some(uuid) => {
                let entity = Entity(*uuid);
                match self.get_component::<Creature>(entity) {
                    Some(creature) => Some((entity, creature)),
                    None => None,
                }
            }
            None => None,
        }
    }
}
