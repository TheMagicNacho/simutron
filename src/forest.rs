use crate::base_terrain::{Environments, Map, MapBuilder, Terrain, Tile};

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
        let default_row = vec![default_tile; width as usize];
        let tiles = vec![default_row; height as usize];

        let environment = Environments::Forest;

        ForestBuilder {
            tile_size,
            tiles,
            environment,
            map_name: None,
            description: None,
        }
    }

    fn add_description(&mut self, description: String) -> &mut Self {
        self.description = Some(description);
        self
    }

    fn add_name(&mut self, map_name: String) -> &mut Self {
        self.map_name = Some(map_name);
        self
    }

    fn add_base_material(&mut self, x: u32, y: u32, tile: Tile<Forest>) -> &mut Self {
        self.tiles[y as usize][x as usize] = tile;
        self
    }

    fn build(&self) -> Map<Forest> {
        Map {
            tile_size: self.tile_size.clone(),
            environment: self.environment.clone(),
            name: self.map_name.clone(),
            description: self.description.clone(),
            tiles: self.tiles.clone(),
            props: vec![],
        }
    }
}
