use crate::creatures::components::CreatureActions;
use crate::creatures::creature_builder::AppendageEffect;
use crate::errors::SimutronError;
use crate::props::components::{PropAction, PropEffect};
use creatures::components::Creature;
use creatures::creature_builder::MorphologyBuilder;
use ecs::components::{Inventory, Position, PropHealth};
use ecs::world::World;
use map::base_terrain::{MapBuilder, Tile};
use map::forest::{ForestBuilder, ForestMaterial};
use props::components::Prop;
use std::error::Error;

mod creatures;
mod ecs;
mod errors;
mod map;
mod props;

// TODO: Make everything async

fn main() {
    // WORLD CREATION
    let mut world = World::new();

    let mut forest_map = ForestBuilder::new(5, 5, 5, Tile::new(ForestMaterial::Soil));
    forest_map.add_base_material(0, 0, Tile::new(ForestMaterial::Leaves));
    forest_map.add_base_material(0, 1, Tile::new(ForestMaterial::Gravel).clone());
    forest_map.add_base_material(0, 2, Tile::new(ForestMaterial::Gravel).clone());
    forest_map.add_base_material(0, 3, Tile::new(ForestMaterial::Gravel).clone());
    forest_map.add_base_material(0, 4, Tile::new(ForestMaterial::Gravel).clone());
    forest_map.add_name("Forest");
    let forest_map = forest_map.build();
    println!("{:#?}", forest_map.clone());
    let forest_map_id = Some(forest_map.id);
    world.add_map(forest_map);

    // EXAMPLE: How to build a morphology
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

    let alice_body = Creature {
        name: "Alice".to_string(),
        corpus: humanoid_structure.clone(),
    };
    world.create_creature(alice_body);

    let bob_body = Creature {
        name: "Bob".to_string(),
        corpus: humanoid_structure.clone(),
    };
    world.create_creature(bob_body);

    // You can create a prop manually using the foundational entities.
    let coin = world.create_entity();
    let coin_prop = Prop {
        name: String::from("Gold Coin"),
        description: String::from("A coin of impressive value."),
    };
    world.add_component(coin, coin_prop); // A prop is a component
    world.add_component(
        coin,
        Position {
            map: forest_map_id,
            x: 0,
            y: 0,
        },
    ); // we can give the prop a Positional component

    // Or you can create a prop using the helper methods.
    let crystal = world.create_prop("Mysterious Crystal", "A crystal of mysterious power.");

    let jar = world.create_entity();
    world.add_component(
        jar,
        Prop {
            name: String::from("Jar"),
            description: String::from("A clay jar."),
        },
    );
    world.add_component(jar, PropHealth { health: 100 });
    world.add_component(
        jar,
        Position {
            map: forest_map_id,
            x: 4,
            y: 0,
        },
    );
    world.add_component(
        jar,
        Inventory {
            items: vec![crystal.get_uuid()],
        },
    );
    // GAME RUN LOOP EXAMPLE
    let alice = world.get_creature_by_name("Alice").unwrap().1.clone();
    println!("Alice's overall health: {}", alice.get_character_health());

    // In an unprovoked turn of events, bob decides to attack alice.
    let swing = CreatureActions {
        from: world.get_creature_by_name("Bob").unwrap().0,
        to: world.get_creature_by_name("Alice").unwrap().0,
        target: "Right Hand".to_string(),
        effect: AppendageEffect::Abrasion,
        impact: -30,
    };
    world.apply_creature_action(&swing).unwrap();

    println!(
        "After attack, Alice's overall health: {}",
        world
            .get_creature_by_name("Alice")
            .unwrap()
            .1
            .get_character_health()
    );
    // Wounded, Alice decides to bandage her own hand.
    let alice = world.get_creature_by_name("Alice").unwrap().0;
    let bandage = CreatureActions {
        from: alice,
        to: alice,
        target: "Right Hand".to_string(),
        effect: AppendageEffect::Abrasion,
        impact: 20,
    };
    world.apply_creature_action(&bandage).unwrap();

    println!(
        "After attack, Alice's overall health: {}",
        world
            .get_creature_by_name("Alice")
            .unwrap()
            .1
            .get_character_health()
    );

    // Meanwhile, Bob spots a jar and decides to smash it, being the brute that he is.
    let smash_jar = PropAction {
        from: world.get_creature_by_name("Bob").unwrap().0,
        to: jar,
        effect: PropEffect::Damage,
        impact: 10,
    };

    world.apply_prop_action(&smash_jar).unwrap();

    println!(
        "Jar's health after being smashed: {}",
        world.get_component::<PropHealth>(jar).unwrap().health
    );

    // Let's give Alice a fancy pack to hold things.
    world.add_component(alice, Inventory::new());
    // Alice decides to inspect the jar.
    let inspect_jar = PropAction {
        from: alice,
        to: jar,
        effect: PropEffect::Inspect,
        impact: 60,
    };
    let jar_copy = world.apply_prop_action(&inspect_jar).unwrap();
    println!("Alice inspects the jar: {:#?}", jar_copy);
    // Woah! She finds a mysterious crystal inside!
    // She takes it out and puts it in her inventory.
    let crystal_prop = world
        .remove_from_inventory(jar, crystal.get_uuid())
        .unwrap();
    println!("Alice takes the crystal from the jar: {:#?}", crystal_prop);
    // She puts it in her own inventory.
    match world.get_component_mut::<Inventory>(alice) {
        Some(inventory) => {
            inventory.items.push(crystal.get_uuid());
        }
        None => {
            panic!("Alice has no inventory to put the crystal in.");
        }
    }

    let alice = world.get_creature_by_name("Alice").unwrap().1.clone();
    println!("Alice's final state: {:#?}", alice);
}
