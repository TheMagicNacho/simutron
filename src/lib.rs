//! Simutron - An RPG Game Engine
//!
//! Simutron is a flexible RPG engine built on an Entity-Component-System (ECS) architecture.
//! It provides tools for creating creatures, maps, props, and managing interactions between them.
//!
//! # Examples
//!
//! ```no_run
//! use simutron::prelude::*;
//!
//! let mut world = World::new();
//! let creature = Creature {
//!     name: "Hero".to_string(),
//!     corpus: MorphologyBuilder::new("Torso").build(),
//! };
//! world.create_creature(creature);
//! ```

// Core modules
pub mod creatures;
pub mod ecs;
pub mod errors;
pub mod map;
pub mod props;

// Prelude module for convenient imports
pub mod prelude {
    // Re-export commonly used types from creatures
    pub use crate::creatures::{
        components::{CreatureActions, CreatureSheet},
        creature_builder::{AppendageEffect, MorphologyBuilder},
        morphologies,
        Creature,
    };

    // Re-export ECS types
    pub use crate::ecs::{
        component::Component,
        components::{Inventory, Position, PropHealth},
        entity::Entity,
        world::World,
    };

    // Re-export error types
    pub use crate::errors::{SimutronError, SimutronResult};

    // Re-export map types
    pub use crate::map::{
        base_terrain::{BaseMap, Environments, Map, MapBuilder, Terrain, Tile},
        forest::{Forest, ForestBuilder, ForestMaterial},
    };

    // Re-export props types
    pub use crate::props::components::{Prop, PropAction, PropEffect};
}
