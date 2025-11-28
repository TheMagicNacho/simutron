use crate::ecs::component::Component;

#[derive(Debug, Clone)]
pub struct Prop {
    pub(crate) name: String,
    pub(crate) description: String,
}

impl Component for Prop {}
