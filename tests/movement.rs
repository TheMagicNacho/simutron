use simutron::prelude::*;

#[test]
fn test_movement() {
    // WORLD CREATION
    let mut world = World::new();

    let mut forest_map = ForestBuilder::new(5, 5, 5, Tile::new(ForestMaterial::Soil));
    forest_map.add_base_material(0, 0, Tile::new(ForestMaterial::Leaves));
    forest_map.add_base_material(0, 1, Tile::new(ForestMaterial::Gravel).clone());
    forest_map.add_base_material(0, 2, Tile::new(ForestMaterial::Gravel).clone());
    forest_map.add_base_material(0, 3, Tile::new(ForestMaterial::Gravel).clone());
    forest_map.add_base_material(0, 4, Tile::new(ForestMaterial::Gravel).clone());
    forest_map.add_name("Forest");
    forest_map.add_description("A serene forest filled with tall trees and the sound of chirping birds. A great start to our world.");
    let forest_map = forest_map.build();
    world.add_map(forest_map.clone());

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
    println!("Humanoid Structure: {:#?}", humanoid_structure);

    let alice_body = Creature {
        name: "Alice".to_string(),
        corpus: humanoid_structure.clone(),
    };
    world.create_creature(alice_body);

    let alice = world
        .get_creature_by_name("Alice")
        .map(|(entity, _)| entity)
        .unwrap();

    let alice_pos = world.get_component::<Position>(alice).unwrap();
    println!("Alice's initial position is: {:#?}", alice_pos);

    let alice_spawn_point = Position {
        map: Some(forest_map.id),
        x: 0,
        y: 4,
    };
    world.teleport_creature(alice, alice_spawn_point).unwrap();

    let alice_pos = world.get_component::<Position>(alice).unwrap();
    println!("Alice's spawn position is: {:#?}", alice_pos);

    let alice_creature_sheet = CreatureSheet {
        speed: 15,
        strength: 5,
        intelligence: 5,
        dexterity: 5,
        constitution: 5,
        wisdom: 5,
        charisma: 5,
    };
    world.add_component(alice, alice_creature_sheet);

    let move_up = Position {
        map: Some(forest_map.id),
        x: 0,
        y: 3,
    };
    let move_right = Position {
        map: Some(forest_map.id),
        x: 1,
        y: 3,
    };
    let move_diagonal = Position {
        map: Some(forest_map.id),
        x: 2,
        y: 2,
    };
    let movement_positions = vec![move_up, move_right, move_diagonal];

    world.move_creature(alice, movement_positions).unwrap();

    let diag_1 = Position {
        map: Some(forest_map.id),
        x: 3,
        y: 1,
    };
    let diag_2 = Position {
        map: Some(forest_map.id),
        x: 4,
        y: 0,
    };
    let movement_positions = vec![diag_1, diag_2];
    world.move_creature(alice, movement_positions).unwrap();

    let alice_pos = world.get_component::<Position>(alice).unwrap();
    println!("Alice's new position is: {:#?}", alice_pos);

    let diag_1 = Position {
        map: Some(forest_map.id),
        x: 3,
        y: 1,
    };
    let diag_2 = Position {
        map: Some(forest_map.id),
        x: 2,
        y: 2,
    };
    let diag_3 = Position {
        map: Some(forest_map.id),
        x: 1,
        y: 3,
    };
    let movement_positions = vec![diag_1, diag_2, diag_3];
    world.move_creature(alice, movement_positions).unwrap();
    let alice_pos = world.get_component::<Position>(alice).unwrap();
    println!("Alice's final position is: {:#?}", alice_pos);
    println!(
        "You'll notice that alice could not complete her movement and only moved the valid step."
    );

    // Verify test assertions
    assert!(
        alice_pos.x == 2 && alice_pos.y == 2,
        "Alice should end at position (2, 2)"
    );
}
