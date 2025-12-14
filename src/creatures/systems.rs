use crate::creatures::Creature;
use crate::creatures::components::CreatureActions;
use crate::creatures::creature_builder::Appendage;
use crate::ecs::entity::Entity;
use crate::ecs::world::World;
use crate::errors::{SimutronError, SimutronResult};
use crate::prelude::{CreatureSheet, Position};
use crate::runtime_error;

impl World {
    pub fn get_creature_by_name(&self, name: &str) -> Option<(Entity, &Creature)> {
        match self.get_creature_id(name) {
            Some(uuid) => {
                let entity = Entity(uuid);
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
    pub fn apply_creature_action(&mut self, action: &CreatureActions) -> SimutronResult<Creature> {
        let creature = self.get_component_mut::<Creature>(action.to);
        if let Some(creature) = creature {
            let root = &mut creature.corpus;
            Self::get_appendage_ref_by_name(root, action.clone());
            Ok(creature.clone())
        } else {
            runtime_error!("Creature not found in world.")
        }
    }
}
