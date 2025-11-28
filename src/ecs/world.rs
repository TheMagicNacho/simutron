use crate::creatures::components::{Creature, CreatureActions};
use crate::creatures::creature_builder::Appendage;
use crate::ecs::component::{Component, ComponentVec};
use crate::ecs::components::{Position, PropHealth};
use crate::ecs::entity::Entity;
use crate::errors::SimutronResult;
use crate::map::base_terrain::{Map, Terrain};
use crate::props::components::Prop;
use crate::SimutronError;
use crate::{runtime_error, PropAction, PropEffect};
use log::{debug, warn};
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

    fn get_appendage_ref_by_name(appendage: &mut Appendage, action: CreatureActions) {
        if appendage.name == action.target {
            appendage.apply_effect(action.effect, action.impact);
            return;
        }
        if let Some(ref mut children) = appendage.connected_to {
            for child in children.iter_mut() {
                Self::get_appendage_ref_by_name(child, action.clone());
            }
        }
    }

    pub(crate) fn apply_creature_action(
        &mut self,
        action: &CreatureActions,
    ) -> SimutronResult<Creature> {
        let creature = self.get_component_mut::<Creature>(action.to);
        if let Some(creature) = creature {
            let root = &mut creature.corpus;
            Self::get_appendage_ref_by_name(root, action.clone());
            Ok(creature.clone())
        } else {
            runtime_error!("Creature not found in world.")
        }
    }
    fn prop_fix(&mut self, action: &PropAction) -> SimutronResult<PropHealth> {
        let health_comp = self.get_component_mut::<PropHealth>(action.to);
        if let Some(health_comp) = health_comp {
            health_comp.health = (health_comp.health as i32 + action.impact).min(100) as u8;
            debug!(
                "Fixed prop {:#?} by {}. New health: {}",
                action.to, action.impact, health_comp.health
            );
            Ok(health_comp.clone())
        } else {
            runtime_error!("Prop {:#?} has no health component to fix.", action.to)
        }
    }

    fn prop_damage(&mut self, action: &PropAction) -> SimutronResult<PropHealth> {
        let health_comp = self.get_component_mut::<PropHealth>(action.to);
        if let Some(health_comp) = health_comp {
            health_comp.health = (health_comp.health as i32 - action.impact).max(0) as u8;
            debug!(
                "Damaged prop {:#?} by {}. New health: {}",
                action.to, action.impact, health_comp.health
            );
            Ok(*health_comp)
        } else {
            runtime_error!("Prop {:#?} has no health component to damage.", action.to)
        }
    }

    pub(crate) fn apply_prop_action(&mut self, action: &PropAction) -> SimutronResult<Prop> {
        let _ = match action.effect {
            PropEffect::Fix => self.prop_fix(action)?,
            PropEffect::Damage => self.prop_damage(action)?,
        };
        let prop = self.get_component::<Prop>(action.to);
        match prop {
            Some(prop) => Ok(prop.clone()),
            None => {
                runtime_error!("Prop {:?} not found in world.", action.to)
            }
        }
    }
}
