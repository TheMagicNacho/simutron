use crate::creatures::creature_builder::{Appendage, AppendageEffect};
use crate::ecs::component::Component;
use crate::ecs::entity::Entity;

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
        let (appendages, total_health) = self.calculate_corpus_health(self.corpus.clone());
        let average_health = if appendages > 0 {
            total_health / (appendages)
        } else {
            0
        };
        average_health as u8
    }

    fn calculate_corpus_health(&self, appendage: Appendage) -> (i32, i32) {
        if appendage.connected_to.is_none() {
            return (1, appendage.get_health().into());
        }
        let mut total_appendages = 1i32; // Count the current appendage
        let mut total_health = appendage.get_health() as i32;
        if let Some(children) = &appendage.connected_to {
            for child in children {
                let (child_count, child_health) = self.calculate_corpus_health(child.clone());
                total_appendages += child_count;
                total_health += child_health;
            }
        }
        (total_appendages, total_health)
    }
}

/// Represents an action taken by a character on another character
#[derive(Clone, Debug)]
pub struct CreatureActions {
    /// Who generated the action. String name of the character.
    pub from: Entity,
    /// To whom is the action meant for.
    pub to: Entity,
    /// Which appendage is the target of the action.
    pub target: String,
    /// What effect the action has on the target appendage.
    pub effect: AppendageEffect,
    /// The health impact of the action on the target appendage.Positive values heal, negative values damage.
    pub impact: i8,
}

// impl Component for CreatureActions {}
