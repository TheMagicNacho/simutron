use simutron::creatures::morphologies::humanoid::humanoid_corpus;
use simutron::prelude::*;

#[test]
fn test_movement() {
    // WORLD CREATION
    let mut world = World::new();

    let alice = Creature {
        name: "Alice".to_string(),
        corpus: humanoid_corpus(),
    };

    let mut forest_map = ForestBuilder::new(5, 5, 5, Tile::new(ForestMaterial::Soil));
    forest_map.add_name("Forest");
    forest_map.add_description("A serene forest filled with tall trees and the sound of chirping birds. A great start to our world.");

    forest_map.add_base_material(0, 0, Tile::new(ForestMaterial::Leaves));
    forest_map.add_base_material(0, 1, Tile::new(ForestMaterial::Gravel).clone());
    forest_map.add_base_material(0, 2, Tile::new(ForestMaterial::Gravel).clone());
    forest_map.add_base_material(0, 3, Tile::new(ForestMaterial::Gravel).clone());
    forest_map.add_base_material(0, 4, Tile::new(ForestMaterial::Gravel).clone());

    let forest_map = forest_map.build();
    world.add_map(forest_map.clone());
}
