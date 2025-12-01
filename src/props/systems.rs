use crate::ecs::components::{Inventory, Position, PropHealth};
use crate::ecs::world::World;
use crate::errors::SimutronResult;
use crate::props::components::{Prop, PropAction, PropEffect};
use crate::{runtime_error, SimutronError};

use crate::ecs::entity::Entity;
use log::debug;
use uuid::Uuid;

impl World {
    /// Apply a prop action to a prop in the world.
    /// Returns a copy of the Prop after the action is applied as a curtesy. You'll need to get a reference if you want to do additional changes.
    pub fn apply_prop_action(&mut self, action: &PropAction) -> SimutronResult<Prop> {
        match action.effect {
            PropEffect::Fix => self.prop_fix(action)?,
            PropEffect::Damage => self.prop_damage(action)?,
            PropEffect::Inspect => self.prop_inspect(action)?,
        };
        let prop = self.get_component::<Prop>(action.to);
        match prop {
            Some(prop) => Ok(prop.clone()),
            None => {
                runtime_error!("Prop {:?} not found in world.", action.to)
            }
        }
    }

    pub(crate) fn add_to_inventory(&mut self, add_to: Entity, item: Uuid) -> SimutronResult<Prop> {
        match self.get_component_mut::<Inventory>(add_to) {
            Some(inventory) => {
                inventory.items.push(item);
                match self.get_component::<Prop>(Entity(item)) {
                    Some(prop) => Ok(prop.clone()),
                    None => {
                        runtime_error!("Item {:?} not found in world.", item)
                    }
                }
            }
            None => {
                runtime_error!(
                    "Entity {:?} has no inventory to take item from. Did you remember to attach an `Inventory` component?",
                    add_to
                )
            }
        }
    }

    pub(crate) fn remove_from_inventory(
        &mut self,
        take_from: Entity,
        item: Uuid,
    ) -> SimutronResult<Prop> {
        match self.get_component_mut::<Inventory>(take_from) {
            Some(inventory) => {
                if let Some(pos) = inventory.items.iter().position(|x| *x == item) {
                    inventory.items.remove(pos);
                    match self.get_component::<Prop>(Entity(item)) {
                        Some(prop) => Ok(prop.clone()),
                        None => {
                            runtime_error!("Item {:?} not found in world.", item)
                        }
                    }
                } else {
                    runtime_error!(
                        "Item {:?} not found in inventory of entity {:?}.",
                        item,
                        take_from
                    )
                }
            }
            None => {
                runtime_error!(
                    "Entity {:?} has no inventory to take item from. Did you remember to attach an `Inventory` component?",
                    take_from
                )
            }
        }
    }

    /// Adds any attached components to the Prop component for inspection.
    /// Allows/expects a function to call the prop once it has mutated.
    fn prop_inspect(&mut self, action: &PropAction) -> SimutronResult<()> {
        let entity = action.to;
        let prop = self.get_component::<Prop>(entity);
        match prop {
            Some(prop) => {
                // get all components of the entity
                let health = self.get_component::<PropHealth>(entity).copied();
                let inventory = self.get_component::<Inventory>(entity).cloned();
                let position = self.get_component::<Position>(entity).copied();

                self.add_component(
                    entity,
                    Prop {
                        name: prop.name.clone(),
                        description: prop.description.clone(),
                        health,
                        inventory,
                        position,
                    },
                );
                Ok(())
            }
            None => {
                runtime_error!("Prop {:?} not found in world.", action.to)
            }
        }
    }

    fn prop_fix(&mut self, action: &PropAction) -> SimutronResult<()> {
        let health_comp = self.get_component_mut::<PropHealth>(action.to);
        if let Some(health_comp) = health_comp {
            health_comp.health = (health_comp.health as i32 + action.impact).min(100) as u8;
            debug!(
                "Fixed prop {:#?} by {}. New health: {}",
                action.to, action.impact, health_comp.health
            );
            Ok(())
        } else {
            runtime_error!("Prop {:#?} has no health component to fix.", action.to)
        }
    }

    fn prop_damage(&mut self, action: &PropAction) -> SimutronResult<()> {
        let health_comp = self.get_component_mut::<PropHealth>(action.to);
        if let Some(health_comp) = health_comp {
            health_comp.health = (health_comp.health as i32 - action.impact).max(0) as u8;
            debug!(
                "Damaged prop {:#?} by {}. New health: {}",
                action.to, action.impact, health_comp.health
            );
            Ok(())
        } else {
            runtime_error!("Prop {:#?} has no health component to damage.", action.to)
        }
    }
}
