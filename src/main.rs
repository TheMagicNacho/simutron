use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

mod creature;

pub trait Terrain: 'static + Debug + Clone + PartialEq {
    type Material: 'static + Debug + Clone + PartialEq;
    // fn default_material() -> Self::Material;
}

#[derive(Debug, Clone, PartialEq)]
enum Environments {
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
#[derive(Debug, Clone, PartialEq)]
pub struct Tile<T: Terrain> {
    pub material: T::Material,
    pub is_blocking: bool,
    pub is_opaque: bool,
    /// base_luminance is the base amount of light on a tile.
    /// Expressed as a value between 0 and 100
    /// luminance can be modified during a run (ex. turning on a lamp)
    pub base_luminance: u8,
    _terrain: PhantomData<T>, // We don't store T, but need to mark it as used.
}

impl<T: Terrain> Tile<T> {
    pub fn new(default_material: T::Material) -> Self {
        Tile {
            material: default_material,
            is_blocking: false,
            is_opaque: false,
            base_luminance: 0,
            _terrain: Default::default(),
        }
    }
    pub fn set_blocking(&mut self, is_blocking: bool) -> &mut Self {
        self.is_blocking = is_blocking;
        self
    }

    pub fn set_opaque(&mut self, is_opaque: bool) -> &mut Self {
        self.is_opaque = is_opaque;
        self
    }
    pub fn set_base_luminance(&mut self, base_luminance: u8) -> &mut Self {
        self.base_luminance = base_luminance;
        self
    }
}

#[derive(Clone, PartialEq)]
pub struct Map<T: Terrain> {
    environment: Environments,
    name: Option<String>,
    description: Option<String>,
    // Tiles are constant and persistent
    tiles: Vec<Vec<Tile<T>>>,
    // _terrain: PhantomData<T>,
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

        // TODO: make a tiles printer so its easier to visualize
        write!(
            f,
            "Map: {:?}\nDescription: {:?}\n{:?}",
            name, description, self.tiles
        )
    }
}

pub trait MapBuilder<T: Terrain> {
    fn new(width: u32, height: u32, default_tile: Tile<T>) -> Self;
    fn add_description(&mut self, description: String) -> &mut Self;
    fn add_name(&mut self, map_name: String) -> &mut Self;
    fn add_base_material(&mut self, x: u32, y: u32, tile: Tile<T>) -> &mut Self;
    fn build(&self) -> Map<T>;
}
// Actual Environments (tsting with forest)
#[derive(Clone, Debug, PartialEq)]
pub enum ForestMaterial {
    Soil,
    Leaves,
    Gravel,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Forest;

impl Terrain for Forest {
    type Material = ForestMaterial;
}

pub struct ForestBuilder {
    tiles: Vec<Vec<Tile<Forest>>>,
    map_name: Option<String>,
    description: Option<String>,
    environment: Environments,
}
// impl Map<Forest> {}

impl MapBuilder<Forest> for ForestBuilder {
    fn new(width: u32, height: u32, default_tile: Tile<Forest>) -> Self {
        let default_row = vec![default_tile; width as usize];
        let tiles = vec![default_row; height as usize];

        let environment = Environments::Forest;

        ForestBuilder {
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
            environment: self.environment.clone(),
            name: self.map_name.clone(),
            description: self.description.clone(),
            tiles: self.tiles.clone(),
        }
    }
}
fn main() {
    let map = ForestBuilder::new(2, 2, Tile::new(ForestMaterial::Soil))
        .add_name(String::from("The 100 Acre Woods"))
        .add_description(String::from("The woods, though lined with dirt moss and detritus, represents a fresh and clean start. The threashold to adventure."))
        .add_base_material(0, 0, Tile::new(ForestMaterial::Leaves))
        .add_base_material(0, 1, Tile::new(ForestMaterial::Gravel).set_blocking(true).clone())
        .build();
    println!("{:?}", map);

    let mut map = ForestBuilder::new(2, 2, Tile::new(ForestMaterial::Soil));
    map.add_base_material(0, 0, Tile::new(ForestMaterial::Leaves));
    map.add_base_material(
        0,
        1,
        Tile::new(ForestMaterial::Gravel).set_blocking(true).clone(),
    );
    map.add_base_material(
        0,
        1,
        Tile::new(ForestMaterial::Gravel)
            .set_opaque(true)
            .set_blocking(true)
            .clone(),
    );
    let map = map.build();
    println!("{:?}", map);

    // EXAMPLE: How to apply an effect on an appendage
    // let mut hand = Appendage::new("Right Hand");
    // println!("{:?}", hand);
    // hand.apply_effect(AppendageEffect::ABRASION, -10);
    // println!("{:?}", hand);
    // hand.apply_effect(AppendageEffect::INFECTED, -30);
    // println!("{:?}", hand);
    // hand.apply_effect(AppendageEffect::ABRASION, 30);
    // println!("{:?}", hand);
    //

    // EXAMPLE: How to build a morphology
    // let mut humanoid = MorphologyBuilder::new("Torso");
    // humanoid.add_appendage("Torso", "Left Arm");
    // humanoid.add_appendage("Torso", "Right Arm");
    // humanoid.add_appendage("Left Arm", "Left Hand");
    // humanoid.add_appendage("Right Arm", "Right Hand");
    // humanoid.add_appendage("Torso", "Left Leg");
    // humanoid.add_appendage("Torso", "Right Leg");
    // humanoid.add_appendage("Left Leg", "Left Foot");
    // humanoid.add_appendage("Right Leg", "Right Foot");
    // humanoid.add_appendage("Torso", "Head");
    //
    // let humanoid_structure = humanoid.build();
    //
    // let mut alice = Creature {
    //     name: "Alice".to_string(),
    //     corpus: humanoid_structure.clone(),
    // };

    // println!("Alice's overall health: {}", alice.get_character_health());

    // EXAMPLE: How to create an action and apply an action on a creature
    // let swing = CreatureActions {
    //     from: "Bob".to_string(),
    //     to: "Alice".to_string(),
    //     target: "Right Hand".to_string(),
    //     effect: AppendageEffect::Abrasion,
    //     impact: -30,
    // };
    // alice.apply_action(swing.clone());
    // println!(
    //     "After attack, Alice's overall health: {}",
    //     alice.get_character_health()
    // );

    // let bandage = CreatureActions {
    //     from: "Alice".to_string(),
    //     to: "Alice".to_string(),
    //     target: "Right Hand".to_string(),
    //     effect: AppendageEffect::Abrasion,
    //     impact: 20,
    // };
    // alice.apply_action(bandage.clone());
    // println!(
    //     "After attack, Alice's overall health: {}",
    //     alice.get_character_health()
    // );

    // println!("{:#?}", alice);
}
