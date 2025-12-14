pub mod forest;

#[derive(Debug, Clone, PartialEq)]
pub enum Environments {
    Forest,
    // Marsh,
    // Hills,
    // Mountains,
    // Urban,
    // Desert,
    // Plains,
    // Aquatic,
    Dungeon,
    // Interior, // I think interior is separate from dungeon, but they might be the same.
}
