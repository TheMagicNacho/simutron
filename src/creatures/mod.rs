use crate::creatures::creature_builder::Appendage;
use crate::ecs::component::Component;

pub mod components;
pub mod creature_builder;
pub mod morphologies;
pub(crate) mod systems;

/// Represents a character with a name and body structure
/// A creature is an entity with a pre-defined set of components.
#[derive(Debug, Clone)]
pub struct Creature {
    pub name: String,
    pub corpus: Appendage,
}

impl Component for Creature {}

impl Creature {
    pub fn get_character_health(&self) -> u8 {
        let (appendages, total_health) = Self::calculate_corpus_health(self.corpus.clone());
        let average_health = if appendages > 0 {
            total_health / (appendages)
        } else {
            0
        };
        average_health as u8
    }

    fn calculate_corpus_health(appendage: Appendage) -> (u32, u32) {
        let mut total_appendages = 1u32;
        if appendage.connected_to.is_none() {
            return (total_appendages, appendage.get_health() as u32);
        }
        let mut total_health = appendage.get_health() as u32;
        if let Some(children) = &appendage.connected_to {
            for child in children {
                let (child_count, child_health) = Self::calculate_corpus_health(child.clone());
                total_appendages += child_count;
                total_health += child_health;
            }
        }
        (total_appendages, total_health)
    }
}
