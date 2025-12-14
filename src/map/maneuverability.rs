#[derive(Clone, Debug, PartialEq)]
pub enum TerrainManeuverability {
    Unrestricted,
    Restricted,
    SeverelyRestricted,
    Impassable,
}

// NOTE: To change the movement costs, update these values here.
impl TerrainManeuverability {
    pub fn cost(&self) -> u8 {
        match self {
            TerrainManeuverability::Unrestricted => 1,
            TerrainManeuverability::Restricted => 2,
            TerrainManeuverability::SeverelyRestricted => 3,
            TerrainManeuverability::Impassable => u8::MAX,
        }
    }
}
