use crate::Prop;
use std::fmt::Debug;
use std::marker::PhantomData;

pub trait Terrain: 'static + Debug + Clone + PartialEq {
    type Material: 'static + Debug + Clone + PartialEq;
    // fn default_material() -> Self::Material;
}

#[derive(Debug, Clone, PartialEq)]
pub enum Environments {
    Forest,
    Marsh,
    Hills,
    Mountains,
    Urban,
    Desert,
    Plains,
    Aquatic,
    Dungeon,
    Interior, // I think interior is separate from dungeon, but they might be the same.
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
    pub(crate) environment: Environments,
    pub(crate) name: Option<String>,
    pub(crate) description: Option<String>,
    /// The unitless size of each tile in the map.
    /// Used for stroy telling and movement calculations, and therefore can be any unit the designer chooses.
    /// For example, if each tile represents a 5 m square, then tile_size would be 5.
    /// Or come up with a fake unit like "dulops" and say each tile is 10 dulops.
    // A tile size is scoped to the map, so we only store the size once instead of in each tile which could become wasteful.
    pub(crate) tile_size: u32,

    // Manifolds
    pub(crate) tiles: Vec<Vec<Tile<T>>>,
    pub(crate) props: Vec<Vec<Option<Prop>>>,
}

impl<T: Terrain> Map<T> {
    pub fn add_prop(&mut self, x: usize, y: usize, prop: Prop) {
        if self.props.len() != self.tiles.len() {
            while self.props.len() < self.tiles.len() {
                let width = self.tiles[self.props.len()].len();
                self.props.push(Vec::with_capacity(width));
            }
        }
        while self.props[y].len() <= x {
            self.props[y].push(None);
        }
        self.props[y][x] = Some(prop);
    }
}

impl<T: Terrain> Debug for Map<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match &self.name {
            Some(name) => name,
            None => "Unnamed Map",
        };

        let description = match &self.description {
            Some(description) => description,
            None => "No description provided.",
        };

        writeln!(f, "Map: {:?}", description)?;
        writeln!(f, "Description: {:?}\n", description)?;

        for y in 0..self.tiles.len() {
            for x in 0..self.tiles[y].len() {
                write!(f, "| {:?} |", self.tiles[y][x].material)?;
            }
            writeln!(f)?;
        }

        writeln!(f)?;
        for y in 0..self.props.len() {
            for x in 0..self.props[y].len() {
                let name = &self.props[y][x];
                match name {
                    Some(prop) => write!(f, "| {:?} |", prop.name)?,
                    None => write!(f, "|   |")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub trait MapBuilder<T: Terrain> {
    fn new(width: u32, height: u32, tile_size: u32, default_tile: Tile<T>) -> Self;
    fn add_description(&mut self, description: String) -> &mut Self;
    fn add_name(&mut self, map_name: String) -> &mut Self;
    fn add_base_material(&mut self, x: u32, y: u32, tile: Tile<T>) -> &mut Self;
    fn build(&self) -> Map<T>;
}
