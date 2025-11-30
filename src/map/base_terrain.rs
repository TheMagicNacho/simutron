use crate::ecs::components::Position;
use crate::ecs::entity::Entity;
use std::collections::HashMap;
use std::fmt::Debug;
use std::marker::PhantomData;
use uuid::Uuid;

pub trait Terrain: 'static + Debug + Clone + PartialEq {
    type Material: 'static + Debug + Clone + PartialEq;
    // fn default_material() -> Self::Material;
}

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

// IDEA: I think that properties like is_blocking and luminance should be part of the Tile struct
#[derive(Debug, Clone, PartialEq)]
pub struct Tile<T: Terrain> {
    pub material: T::Material,
    _terrain: PhantomData<T>, // We don't store T, but need to mark it as used.
}

impl<T: Terrain> Tile<T> {
    pub fn new(default_material: T::Material) -> Self {
        Tile {
            material: default_material,
            _terrain: Default::default(),
        }
    }
    // pub fn set_blocking(&mut self, is_blocking: bool) -> &mut Self {
    //     self.is_blocking = is_blocking;
    //     self
    // }
    //
    // pub fn set_opaque(&mut self, is_opaque: bool) -> &mut Self {
    //     self.is_opaque = is_opaque;
    //     self
    // }
    // pub fn set_base_luminance(&mut self, base_luminance: u8) -> &mut Self {
    //     self.base_luminance = base_luminance;
    //     self
    // }
    //
}

/// The Map is made up of Tiles and Props
/// Tiles represent the base terrain of the map
/// Props represent objects placed on top of the tiles
/// These two manifolds represent the full state of the map when summed together.
#[derive(Clone, PartialEq)]
pub struct Map<T: Terrain> {
    pub environment: Environments,
    pub name: Option<String>,
    pub description: Option<String>,
    /// The unitless size of each tile in the map.
    /// Used for story telling and movement calculations, and therefore can be any unit the designer chooses.
    /// For example, if each tile represents a 5 m square, then tile_size would be 5.
    /// Or come up with a fake unit like "dulops" and say each tile is 10 dulops.
    // A tile size is scoped to the map, so we only store the size once instead of in each tile which could become wasteful.
    pub tile_size: u32,

    // Manifolds
    pub tiles: Vec<Vec<Tile<T>>>,
    pub entities: HashMap<Position, Entity>,
    pub id: Uuid,
}

impl<T: Terrain> Debug for Map<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = self.name.as_deref().unwrap_or("Unnamed Map");
        let description = self
            .description
            .as_deref()
            .unwrap_or("No description provided.");

        writeln!(f, "Map: {}", name)?;
        writeln!(f, "Description: {}\n", description)?;

        if self.tiles.is_empty() || self.tiles[0].is_empty() {
            return writeln!(f, "Map is empty.");
        }

        let height = self.tiles.len();
        let width = self.tiles[0].len();

        // Print column headers
        write!(f, "   ")?; // Padding for row numbers
        for x in 0..width {
            write!(f, " {:^3}", x)?;
        }
        writeln!(f)?;

        // Print rows with row headers
        for y in 0..height {
            write!(f, "{:2}   ", y)?;
            for x in 0..width {
                let material_str = format!("{:?}", self.tiles[y][x].material);
                let initial = material_str.chars().next().unwrap_or(' ');
                write!(f, "{}   ", initial)?;
            }
            // writeln!(f, "|")?;
            writeln!(f, "")?;
        }

        Ok(())
    }
}

pub trait MapBuilder<T: Terrain> {
    fn new(width: u32, height: u32, tile_size: u32, default_tile: Tile<T>) -> Self;
    fn add_description(&mut self, description: &str) -> &mut Self;
    fn add_name(&mut self, map_name: &str) -> &mut Self;
    fn add_base_material(&mut self, x: u32, y: u32, tile: Tile<T>) -> &mut Self;
    fn build(&self) -> Map<T>;
}
