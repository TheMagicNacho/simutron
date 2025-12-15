use crate::creatures::Creature;
use crate::creatures::components::CreatureActions;
use crate::creatures::creature_builder::Appendage;
use crate::ecs::entity::Entity;
use crate::ecs::world::World;
use crate::errors::{SimutronError, SimutronResult};
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
