use base_terrain::{MapBuilder, Tile};
use forest::{ForestBuilder, ForestMaterial};
use log::{debug, trace};

mod base_terrain;
mod creature;
mod forest;

#[derive(Debug, Clone, PartialEq)]
pub struct PropActionReturn {
    /// A vector of items that were removed from the prop and provided to the user. Item is destroyed if not handled.
    returned_items: Vec<Prop>,
    /// Returns a text description of the item in its current state.
    inspection: String,
    /// A list of items that were inspected within the prop.
    inspected_items: Vec<Prop>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PropEffect {
    /// Inspect the prop without modifying it.
    Inspect,
    /// Negatively change the prop's health.
    Attack,
    /// Positively change the prop's health.
    Fix,
    /// Take all the contents of the prop.
    TakeAll,
    /// Take specific content from the prop.
    Take,
    /// Place content into the prop.
    Place,
}

#[derive(Debug, Clone, PartialEq)]
struct PropAction {
    /// Where the action is coming from.
    pub from: String,
    /// Where the action is going to.
    pub to: String,
    /// What effect is being applied.
    pub effect: PropEffect,
    /// The impact of the effect. Positive values heal/fix, negative values damage/take.
    /// The value MUST match the effect type (i.e., positive for Fix, negative for Attack).
    pub impact: i8,
    /// Allows the passage of props as content between the actor and the prop.
    /// When taking, the interface will return the taken items here.
    /// When placing, the interface will provide the items to be placed here.
    pub interface: Prop,
}
#[derive(Debug, Clone, PartialEq)]
struct Prop {
    name: String,
    description: String,
    content: Vec<Prop>,
    health: u8,
    // height: u8,
    // width: u8,
}

impl Prop {
    fn new(
        name: String,
        description: String,
        content: Vec<Prop>,
        // height: u8,
        // width: u8,
    ) -> Self {
        let health = 100; // Default health
        Prop {
            name,
            description,
            content,
            health,
            // height,
            // width,
        }
    }

    /// Inspects the prop, returning its description and contents.
    /// Does not modify the prop.
    fn inspect(&self) -> PropActionReturn {
        let inspection = format!("{}: {}", self.name, self.description);
        let inspected_items = self.content.clone();
        let returned_items = vec![];
        PropActionReturn {
            inspection,
            returned_items,
            inspected_items,
        }
    }

    fn take_all(&mut self) -> PropActionReturn {
        let returned_items = self.content.clone();
        self.content.clear();
        let inspection = format!("You took all items from the {}.", self.name);
        let inspected_items = vec![];
        PropActionReturn {
            returned_items,
            inspection,
            inspected_items,
        }
    }

    fn attack(&mut self, action: PropAction) -> PropActionReturn {
        if action.impact < 0 {
            let damage = action.impact.abs() as u8;
            self.health = self.health.saturating_sub(damage);
        } else {
            debug!("WARNING: Attack action impact is not negative.");
            trace!("{:?}", action);
        }
        // Placeholder implementation
        let inspection = format!(
            "{} attacked the {}. Current health is {}",
            action.from, self.name, self.health
        );
        let returned_items = vec![];
        let inspected_items = vec![];
        PropActionReturn {
            returned_items,
            inspection,
            inspected_items,
        }
    }

    fn fix(&mut self, action: PropAction) -> PropActionReturn {
        if action.impact > 0 {
            let repair = action.impact as u8;
            self.health = self.health.saturating_add(repair);
        } else {
            debug!("WARNING: Fix action impact is not positive.");
            trace!("{:?}", action);
        }
        // Placeholder implementation
        let inspection = format!(
            "{} fixed the {}. Current health is {}",
            action.from, self.name, self.health
        );
        let returned_items = vec![];
        let inspected_items = vec![];
        PropActionReturn {
            returned_items,
            inspection,
            inspected_items,
        }
    }

    fn take(&mut self, action: PropAction) -> PropActionReturn {
        let prop_index = self
            .content
            .iter()
            .position(|p| p.name == action.interface.name);
        let returned_items = if let Some(index) = prop_index {
            vec![self.content.remove(index)]
        } else {
            vec![]
        };
        let inspection = format!(
            "{} took an {} from the {}.",
            action.from, action.interface.name, self.name
        );
        let inspected_items = self.content.clone();
        PropActionReturn {
            returned_items,
            inspection,
            inspected_items,
        }
    }

    fn place(&mut self, action: PropAction) -> PropActionReturn {
        let item_name = action.interface.name.clone();
        self.content.push(action.interface);
        let inspection = format!(
            "{} placed an {} into the {}.",
            action.from, item_name, self.name
        );
        let returned_items = vec![];
        let inspected_items = self.content.clone();
        PropActionReturn {
            returned_items,
            inspection,
            inspected_items,
        }
    }
    pub fn apply_effect(&mut self, action: PropAction) -> PropActionReturn {
        match action.effect {
            PropEffect::Inspect => self.inspect(),
            PropEffect::TakeAll => self.take_all(),
            PropEffect::Take => self.take(action),
            PropEffect::Attack => self.attack(action),
            PropEffect::Fix => self.fix(action),
            PropEffect::Place => self.place(action),
        }
    }
}

fn main() {
    let map = ForestBuilder::new(2, 2, 5, Tile::new(ForestMaterial::Soil))
        .add_name(String::from("The 100 Acre Woods"))
        .add_description(String::from("The woods, though lined with dirt moss and detritus, represents a fresh and clean start. The threshold to adventure."))
        .add_base_material(0, 0, Tile::new(ForestMaterial::Leaves))
        .add_base_material(0, 1, Tile::new(ForestMaterial::Gravel).clone())
        .build();
    println!("{:?}", map);

    let mut map = ForestBuilder::new(2, 2, 5, Tile::new(ForestMaterial::Soil));
    map.add_base_material(0, 0, Tile::new(ForestMaterial::Leaves));
    map.add_base_material(0, 1, Tile::new(ForestMaterial::Gravel).clone());
    map.add_base_material(0, 1, Tile::new(ForestMaterial::Gravel).clone());
    let mut map = map.build();

    let gold_coin = Prop::new(
        "coin".to_string(),
        "A golden coin. Seems like trash to me.".to_string(),
        vec![],
    );

    let rag = Prop::new(
        "Rag".to_string(),
        "A dirty rag. Could be useful...".to_string(),
        vec![],
    );

    let pot = Prop::new(
        String::from("Old Pot"),
        String::from("A rustic old pot, covered in moss and dirt."),
        vec![gold_coin, rag],
    );

    let floor_lamp = Prop::new(
        String::from("Floor Lamp"),
        String::from("A tall floor lamp with a warm glow."),
        vec![],
    );

    map.add_prop(1, 1, pot);
    map.add_prop(0, 0, floor_lamp);
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
