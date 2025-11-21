use creature::{AppendageEffect, Creature, CreatureActions, MorphologyBuilder};
use std::fmt::Debug;

mod creature;

enum ObjectState {
    /// Fully useable object and undamaged.
    INTACT,
    /// An object that is no longer functional and cannot be repaired.
    DESTROYED,
}

/// Objects represent items in the game world that do not have sentience or actions.
/// They do have health and can be damaged or destroyed.
struct GameObject {
    name: String,
    height: u8,
    width: u8,
    length: u8,
    weight: f32,

    description: String,

    health: u8,
    state: ObjectState,
}

/// The terrain is an abstract description of map elements
pub trait Terrain: 'static + Debug + Clone + PartialEq {
    type Material: 'static + Debug + Clone + PartialEq;
    type Feature: 'static + Debug + Clone + PartialEq;
    fn default_material() -> Self::Material;
}

#[derive(Clone, Debug, PartialEq)]
pub struct Forest;

#[derive(Clone, Debug, PartialEq)]
pub enum ForestMaterial {
    Wood,
    Leaves,
    Soil,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ForestFeature {
    Trees,
    Bushes,
    Rocks,
}

impl Terrain for Forest {
    type Material = ForestMaterial;
    type Feature = ForestFeature;

    fn default_material() -> Self::Material {
        ForestMaterial::Soil
    }
}

/// Represents a single cell in the game world grid.
/// It is now generic over a type T that implements the Terrain trait.
#[derive(Debug, Clone, PartialEq)]
pub struct Tile<T: Terrain> {
    pub material: T::Material,
    pub features: Vec<T::Feature>,
    pub elevation: i32,
    pub is_blocking: bool,
    pub is_opaque: bool,
    // _terrain: PhantomData<T>, // We don't store T, but need to mark it as used.
}

impl<T: Terrain> Default for Tile<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Terrain> Tile<T> {
    /// Creates a new tile using the terrain's default material.
    pub fn new() -> Self {
        Tile {
            material: T::default_material(),
            features: Vec::new(),
            elevation: 0,
            is_blocking: false,
            is_opaque: false,
            // _terrain: PhantomData,
        }
    }

    /// Adds a feature to the tile. Note that the feature MUST be of the correct type.
    pub fn add_feature(&mut self, feature: T::Feature) {
        // You can have logic here to update tile properties based on the feature.
        // This would now require a way to inspect the feature, perhaps via another trait.
        // For now, we just add it.
        self.features.push(feature);
    }
}

pub trait Map<T: Terrain> {
    fn new(width: u32, height: u32) -> Self;

    fn dimensions(&self) -> (u32, u32);
    fn get_tile_mut(&mut self, x: u32, y: u32) -> Option<&mut Tile<T>>;
    fn get_tile(&self, x: u32, y: u32) -> Option<&Tile<T>>;
    fn set_tile(&mut self, x: u32, y: u32, tile: Tile<T>);
}

#[derive(Debug)]
pub struct GameMap<T: Terrain> {
    width: u32,
    height: u32,
    tiles: Vec<Tile<T>>,
    // _terrain: PhantomData<T>,
}

impl<T: Terrain> GameMap<T> {
    /// Creates a new empty GameMap of a given size for a specific terrain.
    pub fn new(width: u32, height: u32) -> Self {
        GameMap {
            width,
            height,
            tiles: vec![Tile::new(); (width * height) as usize],
            // _terrain: PhantomData,
        }
    }
    // ... (xy_to_index remains the same)
}

// The generic implementation of the Map trait for our generic GameMap.
impl<T: Terrain> Map<T> for GameMap<T> {
    fn new(width: u32, height: u32) -> Self {
        todo!()
    }

    fn dimensions(&self) -> (u32, u32) {
        todo!()
    }

    fn get_tile_mut(&mut self, x: u32, y: u32) -> Option<&mut Tile<T>> {
        self.tiles.get_mut((y * self.width + x) as usize)
    }

    fn get_tile(&self, x: u32, y: u32) -> Option<&Tile<T>> {
        todo!()
    }

    fn set_tile(&mut self, x: u32, y: u32, tile: Tile<T>) {
        todo!()
    }

    // ... (trait methods here)
}
fn main() {
    // In your generator code...
    let mut forest_map = GameMap::<Forest>::new(2, 2);
    forest_map
        .get_tile_mut(1, 1)
        .unwrap()
        .add_feature(ForestFeature::Trees);

    println!("{:#?}", forest_map);
    // This is an example usage on how to attack an appendage, and how to heal an appendage.
    // let mut hand = Appendage::new("Right Hand");
    // println!("{:?}", hand);
    // hand.apply_effect(AppendageEffect::ABRASION, -10);
    // println!("{:?}", hand);
    // hand.apply_effect(AppendageEffect::INFECTED, -30);
    // println!("{:?}", hand);
    // hand.apply_effect(AppendageEffect::ABRASION, 30);
    // println!("{:?}", hand);
    //

    let mut humanoid = MorphologyBuilder::new("Torso");
    humanoid.add_appendage("Torso", "Left Arm");
    humanoid.add_appendage("Torso", "Right Arm");
    humanoid.add_appendage("Left Arm", "Left Hand");
    humanoid.add_appendage("Right Arm", "Right Hand");
    humanoid.add_appendage("Torso", "Left Leg");
    humanoid.add_appendage("Torso", "Right Leg");
    humanoid.add_appendage("Left Leg", "Left Foot");
    humanoid.add_appendage("Right Leg", "Right Foot");
    humanoid.add_appendage("Torso", "Head");

    let humanoid_structure = humanoid.build();

    let mut alice = Creature {
        name: "Alice".to_string(),
        corpus: humanoid_structure.clone(),
    };

    // println!("Alice's overall health: {}", alice.get_character_health());

    let swing = CreatureActions {
        from: "Bob".to_string(),
        to: "Alice".to_string(),
        target: "Right Hand".to_string(),
        effect: AppendageEffect::Abrasion,
        impact: -30,
    };

    alice.apply_action(swing.clone());
    // println!(
    //     "After attack, Alice's overall health: {}",
    //     alice.get_character_health()
    // );

    let bandage = CreatureActions {
        from: "Alice".to_string(),
        to: "Alice".to_string(),
        target: "Right Hand".to_string(),
        effect: AppendageEffect::Abrasion,
        impact: 20,
    };

    alice.apply_action(bandage.clone());
    // println!(
    //     "After attack, Alice's overall health: {}",
    //     alice.get_character_health()
    // );

    // println!("{:#?}", alice);
}
