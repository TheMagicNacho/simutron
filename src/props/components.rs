use crate::ecs::component::Component;
use crate::ecs::entity::Entity;

// prop
pub enum PropEffect {
    Fix,
    Damage,
    // Open,
    Inspect,
    // TakeItem,
}
pub struct PropAction {
    pub from: Entity,
    pub to: Entity,
    pub effect: PropEffect,
    pub impact: i32,
}

#[derive(Debug, Clone)]
pub struct Prop {
    pub name: String,
    pub description: String,
}

impl Component for Prop {}
