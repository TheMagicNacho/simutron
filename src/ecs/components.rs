use crate::ecs::component::Component;
use uuid::Uuid;

#[derive(Debug, Copy, Clone)]
pub struct PropHealth {
    pub(crate) health: u8,
}

impl Component for PropHealth {}
impl PropHealth {
    pub fn new(starting_health: u8) -> Self {
        Self {
            health: starting_health,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub struct Position {
    pub(crate) map: Option<Uuid>,
    pub(crate) x: u32,
    pub(crate) y: u32,
}

impl Component for Position {}

/// Call it a bag, a pack, a satchel, or a knapsack WHATEVER YOU WANT!
/// This allows an entity to hold other entities inside it.
/// The entity still belongs to the world, but logically, it is referenced here for usage.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Inventory {
    pub items: Vec<Uuid>,
}
impl Component for Inventory {}
impl Inventory {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }
}
