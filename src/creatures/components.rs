use crate::creatures::creature_builder::AppendageEffect;
use crate::ecs::component::Component;
use crate::ecs::entity::Entity;

/// The stats that define a creature's capabilities.
#[derive(Clone, Debug)]
pub struct CreatureSheet {
    pub speed: u32,
    pub strength: u8,
    pub intelligence: u8,
    pub dexterity: u8,
    pub constitution: u8,
    pub wisdom: u8,
    pub charisma: u8,
}
impl Component for CreatureSheet {}

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
