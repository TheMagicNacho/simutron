use simutron::prelude::*;

#[test]
fn test_simutron_integration() {
    // WORLD CREATION
    println!("Welcome to Simutron!");
    println!("Our first step is to create a map");
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
    println!("Now here's what our map looks like:");
    println!("{:#?}", forest_map.clone());
    println!("Great! Now let's add this map to the world.");
    world.add_map(forest_map.clone());

    println!("Next, let's create a morphology for our creatures.");
    println!("A Morphology defines the structure of a creature's body, including its appendages.");
    // First you create the root appendage.
    let mut humanoid = MorphologyBuilder::new("Torso");
    // Then you add appendages to it.
    // Each appendage has its own state and health value. More on that later.
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
    println!("Here's our humanoid morphology:");
    println!("{:#?}", humanoid_structure);

    println!("Now let's create some creatures using this morphology.");
    let alice_body = Creature {
        name: "Alice".to_string(),
        corpus: humanoid_structure.clone(),
    };
    world.create_creature(alice_body);

    // Everything is an entity.
    // Creatures are entities with a Creature and Position component automatically added.
    // We could manually create an entity and add those components ourselves if we wanted.
    let bob_body = Creature {
        name: "Bob".to_string(),
        corpus: humanoid_structure.clone(),
    };
    world.create_creature(bob_body);

    println!("Props are entities in the world that creatures can interact with.");
    println!("Let's create some props and add them to the world.");
    // You can create a prop manually to limit the components that are attached to the prop like so:
    // 1. Create the entity
    let coin = world.create_entity();
    // 2. Create the prop component
    let coin_prop = Prop::new("Gold Coin", "A shiny gold coin. It looks valuable.");
    // 3. add the prop component to the entity
    world.add_component(coin, coin_prop);

    // Alternatively, you can use the helper function to create a prop with default components.
    let crystal = world.create_prop("Mysterious Crystal", "A crystal of mysterious power.");
    // This comes with Health, Position, and Inventory components by default.
    let jar = world.create_prop(
        "Clay Jar",
        "A sturdy jar that seems to be made of an sturdy material.",
    );
    // Let us add the crystal and the coin to the jar's inventory.
    // The important thing to note here is that we can overwrite components on entities at any time.
    // Components are stored as hashmaps internally, so adding a component that already exists simply overwrites it.
    println!(
        "Jar inventory before adding props has {:#?} items.",
        world.get_component::<Inventory>(jar).unwrap().items.len()
    );

    world.add_component(
        jar,
        Inventory {
            items: vec![crystal.get_uuid(), coin.get_uuid()],
        },
    );
    let jar_inventory = world.get_component::<Inventory>(jar).unwrap();
    println!("but now, the jar contains: ");
    for item_id in &jar_inventory.items {
        let item_prop = world.get_component::<Prop>(Entity(*item_id)).unwrap();
        println!("A {}, {}", item_prop.name, item_prop.description);
    }
    println!();

    println!("Amazing! We've built our world and filled it with creatures and props.");
    println!("Let's see how our creatures can interact with each other and the props.");

    println!();
    let alice = world.get_creature_by_name("Alice").unwrap().1.clone();
    println!(
        "Alice is an amazing witch with a total health of: {}",
        alice.get_character_health()
    );

    println!(
        "Bob, being the aggressive type, decides to attack Alice by swinging at her right hand."
    );
    // Create a Creature action to perform some sort of effect on a target appendage.
    let swing = CreatureActions {
        from: world.get_creature_by_name("Bob").unwrap().0,
        to: world.get_creature_by_name("Alice").unwrap().0,
        target: "Right Hand".to_string(),
        effect: AppendageEffect::Abrasion,
        impact: -30,
    };
    // then apply it.
    world.apply_creature_action(&swing).unwrap();

    println!(
        "'That wretched brute!', Alice thought. She realizes her health is now: {}",
        world
            .get_creature_by_name("Alice")
            .unwrap()
            .1
            .get_character_health()
    );

    println!("Alice quickly decides to bandage her wounded hand.");
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
        "Phew, luckily Alice paid attention during healing potions class. Her health is now: {}",
        world
            .get_creature_by_name("Alice")
            .unwrap()
            .1
            .get_character_health()
    );

    println!("Meanwhile, Bob spots a jar on a nearby rock.");
    println!("Curious, he decides to smash it open.");
    let smash_jar = PropAction {
        from: world.get_creature_by_name("Bob").unwrap().0,
        to: jar,
        effect: PropEffect::Damage,
        impact: 1,
    };
    world.apply_prop_action(&smash_jar).unwrap();

    println!("Not one to be deterred, Alice dawns on a cool backpack.");
    // Inventory allows entities to hold props.
    world.add_component(alice, Inventory::new());

    // get the jar's inventory and create a list of its contents

    let inspect_jar = PropAction {
        from: alice,
        to: jar,
        effect: PropEffect::Inspect,
        impact: 0,
    };
    let jar_inspection = world.apply_prop_action(&inspect_jar).unwrap();
    println!("Alice inspects the jar... {:#?}", jar_inspection);

    println!();
    println!("WOAH! She notices a mysterious crystal inside the jar.");

    println!("So, she decides to take it out and put it in her backpack.");

    let crystal_prop = world
        .remove_from_inventory(jar, crystal.get_uuid())
        .unwrap();
    println!(
        "Alice takes the crystal from the jar... {:#?}",
        crystal_prop
    );
    world.add_to_inventory(alice, crystal.get_uuid()).unwrap();

    let alice_inventory = world.get_component::<Inventory>(alice).unwrap();
    println!(
        "She puts the crystal in her backpack. Her inventory now contains {:#} item(s).",
        alice_inventory.items.len()
    );
    // This is a way to prove/show how to get a prop given an entity id.
    let crystal_in_alice = world
        .get_component::<Prop>(Entity(alice_inventory.items[0]))
        .unwrap();
    println!("Which is... {:#?}", crystal_in_alice);

    println!();
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
