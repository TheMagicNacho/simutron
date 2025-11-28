use crate::ecs::component::Component;
use uuid::Uuid;

#[derive(Debug, Copy, Clone)]
pub struct PropHealth {
    pub(crate) health: u8,
}

impl Component for PropHealth {}

#[derive(Debug)]
pub struct Position {
    pub(crate) map: Option<Uuid>,
    pub(crate) x: u32,
    pub(crate) y: u32,
}

impl Component for Position {}

#[derive(Debug)]
pub struct Inventory {
    pub(crate) content: Vec<Uuid>,
}

impl Component for Inventory {}
