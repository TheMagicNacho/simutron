use crate::ecs::component::Component;
use crate::ecs::components::{Inventory, Position, PropHealth};
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
    pub health: Option<PropHealth>,
    pub inventory: Option<Inventory>,
    pub position: Option<Position>,
}
impl Component for Prop {}
impl Prop {
    pub fn new(name: &str, description: &str) -> Self {
        Prop {
            name: String::from(name),
            description: String::from(description),
            health: None,
            inventory: None,
            position: None,
        }
    }
}
