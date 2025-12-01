use crate::map::base_terrain::{BaseMap, Environments, MapBuilder, Terrain, Tile};
use std::collections::HashMap;
use uuid::Uuid;

// Actual Environments (testing with forest)
#[derive(Clone, Debug, PartialEq)]
pub enum ForestMaterial {
    Soil,
    Leaves,
    Gravel,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Forest;

pub struct ForestBuilder {
    id: Uuid,
    tiles: Vec<Vec<Tile<Forest>>>,
    map_name: Option<String>,
    description: Option<String>,
    environment: Environments,
    tile_size: u32,
}

impl Terrain for Forest {
    type Material = ForestMaterial;
}

impl MapBuilder<Forest> for ForestBuilder {
    fn new(width: u32, height: u32, tile_size: u32, default_tile: Tile<Forest>) -> Self {
        let id = Uuid::new_v4();
        let default_row = vec![default_tile; width as usize];
        let tiles = vec![default_row; height as usize];

        let environment = Environments::Forest;

        ForestBuilder {
            id,
            tile_size,
            tiles,
            environment,
            map_name: None,
            description: None,
        }
    }

    fn add_description(&mut self, description: &str) -> &mut Self {
        let description = String::from(description);
        self.description = Some(description);
        self
    }

    fn add_name(&mut self, map_name: &str) -> &mut Self {
        let map_name = String::from(map_name);
        self.map_name = Some(map_name);
        self
    }

    fn add_base_material(&mut self, x: u32, y: u32, tile: Tile<Forest>) -> &mut Self {
        self.tiles[y as usize][x as usize] = tile;
        self
    }

    fn build(&self) -> BaseMap<Forest> {
        BaseMap {
            id: self.id,
            scale: self.tile_size,
            environment: self.environment.clone(),
            name: self.map_name.clone(),
            description: self.description.clone(),
            entities: HashMap::new(),
            tiles: self.tiles.clone(),
        }
    }
    fn get_tile_size(&self) -> u32 {
        self.tile_size
    }
}
