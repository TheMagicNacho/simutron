use crate::ecs::components::Position;
use crate::ecs::entity::Entity;
use std::collections::HashMap;
use std::fmt::Debug;
use uuid::Uuid;

#[repr(u8)]
#[derive(Debug, Clone, PartialEq)]
/// There are three types of maneuverability for terrain:
/// Unrestricted: No penalty to movement.
/// Restricted: Minor penalty to movement.
/// HighlyRestricted: Major penalty to movement.
/// Blocking: Movement is not possible.
pub(crate) enum Maneuverability {
    Unrestricted = 1,
    Restricted = 2,
    HighlyRestricted = 3,
    Blocking = u8::MAX,
}

impl Maneuverability {
    /// Returns the modifier associated with the maneuverability level.
    /// Use this to calculate movement costs.
    pub fn get_modifier(&self) -> u8 {
        self.clone() as u8
    }
}

pub(crate) trait MaterialManeuverability: 'static + Send + Sync {
    fn get_maneuverability(&self) -> Maneuverability;
}

pub trait Terrain: 'static + Debug + Clone + PartialEq {
    type Material: 'static + Debug + Clone + PartialEq + MaterialManeuverability;
    // type Maneuverability: 'static + Debug + Clone + PartialEq;
    // fn default_material() -> Self::Material;
}

pub trait Map {
    fn get_scale(&self) -> u32;
    fn get_width(&self) -> u32;
    fn get_height(&self) -> u32;
    fn get_maneuverability(&self, position: Position) -> Option<Maneuverability>;
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
    // _terrain: PhantomData<T>, // We don't store T, but need to mark it as used.
}

impl<T: Terrain> Tile<T> {
    pub fn new(default_material: T::Material) -> Self {
        Tile {
            material: default_material,
            // _terrain: Default::default(),
        }
    }
}
/// The Map is made up of Tiles and Props
/// Tiles represent the base terrain of the map
/// Props represent objects placed on top of the tiles
/// These two manifolds represent the full state of the map when summed together.
#[derive(Clone, PartialEq)]
pub struct BaseMap<T: Terrain> {
    pub environment: Environments,
    pub name: Option<String>,
    pub description: Option<String>,
    /// The unitless size of each tile in the map.
    /// Used for story telling and movement calculations, and therefore can be any unit the designer chooses.
    /// For example, if each tile represents a 5 m square, then tile_size would be 5.
    // A tile size is scoped to the map, so we only store the size once instead of in each tile which could become wasteful.
    pub scale: u32,

    // Manifolds
    pub tiles: Vec<Vec<Tile<T>>>,
    pub entities: HashMap<Position, Entity>,
    pub id: Uuid,
}

impl<T: Terrain> Map for BaseMap<T> {
    fn get_scale(&self) -> u32 {
        self.scale
    }
    fn get_width(&self) -> u32 {
        if self.tiles.is_empty() {
            0
        } else {
            self.tiles[0].len() as u32
        }
    }
    fn get_height(&self) -> u32 {
        self.tiles.len() as u32
    }
    fn get_maneuverability(&self, position: Position) -> Option<Maneuverability> {
        let x = position.x;
        let y = position.y;
        self.tiles.get(y as usize).and_then(|row| {
            row.get(x as usize)
                .map(|tile| tile.material.get_maneuverability())
        })
    }
    // fn get_tile(&self, x: u32, y: u32) -> Option<&dyn MaterialManeuverability> {
    //     self.tiles.get(y as usize).and_then(|row| row.get(x as usize))
    // }
}

impl<T: Terrain> Debug for BaseMap<T> {
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
            writeln!(f)?;
        }

        Ok(())
    }
}

pub trait MapBuilder<T: Terrain> {
    fn new(width: u32, height: u32, tile_size: u32, default_tile: Tile<T>) -> Self;
    fn add_description(&mut self, description: &str) -> &mut Self;
    fn add_name(&mut self, map_name: &str) -> &mut Self;
    fn add_base_material(&mut self, x: u32, y: u32, tile: Tile<T>) -> &mut Self;
    fn build(&self) -> BaseMap<T>;
    fn get_tile_size(&self) -> u32;
}
