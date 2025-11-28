use crate::creatures::components::CreatureActions;
use crate::creatures::creature_builder::{Appendage, AppendageEffect};
use crate::ecs::entity::Entity;
use creatures::components::Creature;
use creatures::creature_builder::MorphologyBuilder;
use ecs::components::{Health, Inventory, Position};
use ecs::world::World;
use log::debug;
use map::base_terrain::{MapBuilder, Tile};
use map::forest::{ForestBuilder, ForestMaterial};
use props::components::Prop;

mod creatures;
mod ecs;
mod map;
mod props;

fn get_appendage_ref_by_name(appendage: &mut Appendage, action: CreatureActions) {
    if appendage.name == action.target {
        appendage.apply_effect(action.effect, action.impact);
        return;
    }
    if let Some(ref mut children) = appendage.connected_to {
        for child in children.iter_mut() {
            get_appendage_ref_by_name(child, action.clone());
        }
    }
}

fn apply_creature_action(world: &mut World, action: &CreatureActions) {
    let creature = world.get_component_mut::<Creature>(action.to);
    if let Some(creature) = creature {
        let root = &mut creature.corpus;
        get_appendage_ref_by_name(root, action.clone());
        return;
    } else {
        debug!("Creature {:#?} not found in world.", action.to);
    }
}
fn main() {
    // WORLD CREATION
    let mut world = World::new();

    let mut forest_map = ForestBuilder::new(5, 5, 5, Tile::new(ForestMaterial::Soil));
    forest_map.add_base_material(0, 0, Tile::new(ForestMaterial::Leaves));
    forest_map.add_base_material(0, 1, Tile::new(ForestMaterial::Gravel).clone());
    forest_map.add_base_material(0, 1, Tile::new(ForestMaterial::Gravel).clone());
    forest_map.add_name("Forest");
    let forest_map = forest_map.build();
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
            x: 1,
            y: 1,
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
    world.add_component(jar, Health { health: 100 });
    world.add_component(
        jar,
        Inventory {
            content: vec![crystal.get_uuid()],
        },
    );
    // GAME RUN LOOP EXAMPLE
    let alice = world.get_creature_by_name("Alice").unwrap().1.clone();
    println!("Alice's overall health: {}", alice.get_character_health());

    // EXAMPLE: How to create an action and apply an action on a creature

    let swing = CreatureActions {
        from: world.get_creature_by_name("Bob").unwrap().0,
        to: world.get_creature_by_name("Alice").unwrap().0,
        target: "Right Hand".to_string(),
        effect: AppendageEffect::Abrasion,
        impact: -30,
    };
    apply_creature_action(&mut world, &swing);

    println!(
        "After attack, Alice's overall health: {}",
        world
            .get_creature_by_name("Alice")
            .unwrap()
            .1
            .get_character_health()
    );

    let alice = world.get_creature_by_name("Alice").unwrap().0;
    let bandage = CreatureActions {
        from: alice,
        to: alice,
        target: "Right Hand".to_string(),
        effect: AppendageEffect::Abrasion,
        impact: 20,
    };
    apply_creature_action(&mut world, &bandage);
    println!(
        "After attack, Alice's overall health: {}",
        world
            .get_creature_by_name("Alice")
            .unwrap()
            .1
            .get_character_health()
    );
}
