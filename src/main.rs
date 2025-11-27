use base_terrain::{MapBuilder, Tile};
use forest::{ForestBuilder, ForestMaterial};
use std::any::Any;

mod base_terrain;
mod creature;
mod forest;

#[derive(Debug)]
pub struct Prop {
    name: String,
    description: String,
};

pub trait Component: Any + Sync + Send {}

#[derive(Debug)]
struct Health {
    health: u8,
}
impl Component for Health {}

#[derive(Debug)]
struct Inventory {
    contents: Vec<Prop>,
}
impl Component for Inventory {}

#[derive(Debug)]
struct Position {
    x: u32,
    y: u32,
}
impl Component for Position {}

fn main() {
    let mut map = ForestBuilder::new(2, 2, 5, Tile::new(ForestMaterial::Soil));
    map.add_base_material(0, 0, Tile::new(ForestMaterial::Leaves));
    map.add_base_material(0, 1, Tile::new(ForestMaterial::Gravel).clone());
    map.add_base_material(0, 1, Tile::new(ForestMaterial::Gravel).clone());
    let mut map = map.build();

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
