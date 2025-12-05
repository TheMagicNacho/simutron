use crate::creatures::components::CreatureSheet;
use crate::creatures::Creature;
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
    components: HashMap<TypeId, ComponentVec>,
    maps: HashMap<Uuid, Box<dyn Map>>,
    // A lookup table for creature names to their entity IDs allows for O(1) retrieval
    creature_lookup: HashMap<String, Uuid>,
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

    // A method to move a creature to a new position.
    // Input: Creature entity, movement requests as a vector. Each tile must be adjacent to the previous tile. The movement of all tiles must be less than or equal to the creature's speed stat.
    // Output: Result with updated Creature or error. Also mutate the char's position component.
    // Constraints: The new position must be valid within the map bounds. The new position is valid ONLY if it adheres to the movement rules.
    // Edge: Must calculate diagonal movement correctly.
    pub fn move_creature(
        &mut self,
        creature: Entity,
        requested_positions: Vec<Position>,
    ) -> SimutronResult<Entity> {
        // Movement rules:
        // 1. A creature can move the space less than or equal to its speed stat considering the tile size of a map.
        // 2. When moving diagonally, the creature moves at 1.4x speed cost.
        // 3. All requested positions must be adjacent to the previous position.
        // Is the entity a creature?
        let creature_component = match self.get_component::<Creature>(creature) {
            Some(c) => c.clone(),
            None => return runtime_error!("Entity is not a creature."),
        };
        // get the current position
        let starting_position = match self.get_component::<Position>(creature) {
            Some(pos) => pos.clone(),
            None => return runtime_error!("Creature has no position component."),
        };
        let creature_sheet = match self.get_component::<CreatureSheet>(creature) {
            Some(sheet) => sheet.clone(),
            None => return runtime_error!("Creature has no Creature Sheet component."),
        };

        let position = self.calculate_final_position(
            requested_positions,
            creature_sheet.speed,
            starting_position,
        )?;
        // Update the creature's position component
        self.add_component(creature, position);

        Ok(creature)
    }

    fn calculate_final_position(
        &mut self,
        requested_positions: Vec<Position>,
        available_movement: u32,
        init_position: Position,
    ) -> SimutronResult<Position> {
        let mut init_position = init_position;
        let mut available_movement = available_movement;
        for (index, final_position) in requested_positions.iter().enumerate() {
            // Check adjacency
            let dx = (final_position.x as i32 - init_position.x as i32).abs();
            let dy = (final_position.y as i32 - init_position.y as i32).abs();
            if dx > 1 || dy > 1 || (dx == 0 && dy == 0) {
                return runtime_error!("Requested position is not adjacent to previous position.");
            }
            // TODO: Memoize the map scale if the map is the same as the last loop.
            let current_map_scale = match &final_position.map {
                Some(map_id) => self.maps.get(map_id).unwrap().get_scale(),
                None => return runtime_error!("Creature is not on a map."),
            };
            // First diagonal move costs 1 square, the second diagonal move costs 2 squares, then it repeats.
            // Therefore: if even 1, if odd cost * 2
            let base_cost = if dx == 1 && dy == 1 {
                if (index & 1) == 0 {
                    current_map_scale
                } else {
                    current_map_scale << 1
                }
            } else {
                current_map_scale // Orthogonal movement is the cost
            };

            if available_movement >= base_cost {
                available_movement -= base_cost;
                init_position = *final_position;
            } else {
                // Not enough movement left to proceed to the next position
                // TODO: Verify that this is the correct position to return, may need to return the previous position instead
                return Ok(init_position.to_owned());
            }
        }
        // get the last position in the requested positions
        let last_position = *requested_positions.last().unwrap();
        Ok(last_position.to_owned())
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
