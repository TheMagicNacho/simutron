use crate::creatures::creature_builder::Appendage;
use crate::prelude::MorphologyBuilder;

pub fn build_humanoid_morphology() -> Appendage {
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

    humanoid.build()
}
