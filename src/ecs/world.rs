use crate::creatures::Creature;
use crate::creatures::components::CreatureSheet;
use crate::ecs::component::{Component, ComponentVec};
use crate::ecs::components::{Inventory, Position, PropHealth};
use crate::ecs::entity::Entity;
use crate::errors::{SimutronError, SimutronResult};
use crate::map::base_terrain::{BaseMap, Map, Terrain};
use crate::props::components::Prop;
use crate::runtime_error;
use log::warn;
use std::any::TypeId;
use std::collections::HashMap;
use uuid::Uuid;

// The World holds all entities and components
pub struct World {
    // next_entity_id: u64,
    pub(crate) components: HashMap<TypeId, ComponentVec>,
    pub(crate) maps: HashMap<Uuid, Box<dyn Map>>,
    // A lookup table for creature names to their entity IDs allows for O(1) retrieval
    pub(crate) creature_lookup: HashMap<String, Uuid>,

    // A lookup table for positions to entities at that position. Entities own their own Position component, but this allows for quick spatial queries.
    position_lookup: HashMap<Position, Vec<Entity>>,
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
            position_lookup: HashMap::new(),
        }
    }

    pub fn get_creature_id(&self, name: &str) -> Option<Uuid> {
        self.creature_lookup.get(name).cloned()
    }

    pub fn add_map<T: Terrain>(&mut self, map: BaseMap<T>) {
        if self.maps.contains_key(&map.id) {
            warn!(
                "Map id {} already exists. I WILL OVERWRITE WITH NEW MAP.",
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

    /// Teleport a creature to a new position without any movement rules.
    /// Use this sparingly, as it bypasses all movement constraints.
    /// Mainly useful for fast travel, respawning, or debugging.
    pub fn teleport_creature(
        &mut self,
        creature: Entity,
        new_position: Position,
    ) -> SimutronResult<Entity> {
        let _creature_component = match self.get_component::<Creature>(creature) {
            Some(c) => c.clone(),
            None => return runtime_error!("Entity is not a creature."),
        };
        // validate that the new position is within map bounds
        let map_id = match &new_position.map {
            Some(id) => id,
            None => return runtime_error!("New position has no map assigned."),
        };
        let map = match self.maps.get(map_id) {
            Some(m) => m,
            None => return runtime_error!("Map not found for new position."),
        };
        let map_width = map.get_width();
        let map_height = map.get_height();
        if new_position.x >= map_width || new_position.y >= map_height {
            return runtime_error!("New position is out of map bounds.");
        }

        self.add_component(creature, new_position);
        Ok(creature)
    }

    pub fn create_creature(&mut self, creature: Creature) -> Entity {
        // TODO: Apply the creature systems (health calculations, effect application, personality)
        let new_entity = self.create_entity();
        self.creature_lookup
            .insert(creature.name.clone(), new_entity.0);
        self.add_component(new_entity, creature);

        self.add_component(
            new_entity,
            CreatureSheet {
                speed: 0,
                strength: 0,
                intelligence: 0,
                dexterity: 0,
                constitution: 0,
                wisdom: 0,
                charisma: 0,
            },
        );
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
        let new_prop = Prop::new(prop_name, prop_description);
        self.add_component(new_entity, new_prop);
        self.add_component(new_entity, PropHealth::new(100));
        self.add_component(new_entity, Inventory::new());
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
}
