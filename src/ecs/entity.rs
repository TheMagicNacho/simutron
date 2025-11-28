use uuid::Uuid;

// Entity is just a unique ID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Entity(pub Uuid);

impl Entity {
    pub fn get_uuid(&self) -> Uuid {
        self.0
    }
}
