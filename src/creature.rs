use std::collections::HashMap;

/// Describes the state of an appendage.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum AppendageHealth {
    /// Full health of the appendage with no impairments.
    Full,
    /// Minor injuries which might degrate performance depending on the effect history.
    Wounded,
    /// The appendage is severely impaired and non-functional. No healing is possible, yet remains as a disfigured limb.
    Disabled,
    ///  The appendage has been completely removed or lost. No healing is possible.
    Amputated,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum AppendageEffect {
    /// There is a cut or laceration. Heals on its own over time or with treatment.`
    Abrasion,
    /// Bones are broken. Requires medical treatment to heal.
    Crush,
    ///  The appendage is burned. Heals over time, but will leave a scar if not treated.
    Burn,
    /// Requires an antidote to heal. Will worsen over time without treatment.
    Poison,
    /// The appendage is infected. Requires antibiotics to heal. Will worsen over time without treatment.
    Infect,
}

/// An appendage represents an external body part of a character. Characters can have multiple appendages, each with their own health and state.
/// More humanoid characters will have standard appendages like arms and legs, while more exotic characters may have unique appendages like tails or wings.
/// Each appendage tracks its own health, state, and history of effects that have impacted it.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Appendage {
    /// The name of the appendage (e.g., "Left Arm", "Right Leg")
    name: String,
    /// The state of the appendage based on health.
    state: AppendageHealth,
    /// A log of significant events affecting the appendage.
    /// Stored as a tupel of Effect and the health impact it had on the appendage.
    effect_history: HashMap<AppendageEffect, i8>,
    /// An appendage is connected to something else ( Torso -> Head, leg -> foot, Arm -> Hand )
    connected_to: Option<Vec<Appendage>>,
}

impl Appendage {
    fn new(name: &str) -> Self {
        Appendage {
            name: name.to_string(),
            state: AppendageHealth::Full,
            effect_history: HashMap::new(),
            connected_to: None,
        }
    }

    fn get_health(&self) -> i8 {
        let mut total_health: i8 = 100;
        for impact in &self.effect_history {
            total_health += impact.1;
        }
        total_health
    }

    fn calculate_state(&mut self) {
        let total_health = self.get_health();
        // idea: make this configurable per appendage type
        self.state = if total_health >= 99 {
            AppendageHealth::Full
        } else if total_health >= 30 {
            AppendageHealth::Wounded
        } else if total_health > 15 {
            AppendageHealth::Disabled
        } else {
            // The idea is that once an appendage is too "damaged", it cannot be healed back to functionality.
            AppendageHealth::Amputated
        };
    }

    fn apply_effect(&mut self, attack_effect: AppendageEffect, health_impact: i8) {
        let entry = self.effect_history.entry(attack_effect).or_insert(0);
        *entry += health_impact;
        // Clamp the health impact to not exceed above 100%
        if *entry > 0 {
            *entry = 0;
        }
        self.calculate_state();
    }
}

/// Builder for creating a morphology tree structure
#[derive(Debug, Clone)]
pub struct MorphologyBuilder {
    root: Appendage,
}

impl MorphologyBuilder {
    pub fn new(root_name: &str) -> Self {
        MorphologyBuilder {
            root: Appendage::new(root_name),
        }
    }

    pub fn add_appendage(&mut self, parent_name: &str, child_name: &str) {
        Self::add_to_tree(&mut self.root, parent_name, child_name);
    }

    fn add_to_tree(current: &mut Appendage, parent_name: &str, child_name: &str) {
        if current.name == parent_name {
            let new_appendage = Appendage::new(child_name);
            if let Some(ref mut children) = current.connected_to {
                children.push(new_appendage);
            } else {
                current.connected_to = Some(vec![new_appendage]);
            }
            return;
        }

        if let Some(ref mut children) = current.connected_to {
            for child in children.iter_mut() {
                Self::add_to_tree(child, parent_name, child_name);
            }
        }
    }

    pub fn build(&self) -> Appendage {
        self.root.clone()
    }
}

enum CharacterClass {
    Fighter,
    Rogue,
    Mage,
    Bard,
}

/// Represents an action taken by a character on another character
#[derive(Clone, Debug)]
pub struct CreatureActions {
    /// Who generated the action. String name of the character.
    pub from: String,
    /// Where is the action amied for.
    pub to: String,
    /// Which appendage is the target of the action.
    pub target: String,
    /// What effect the action has on the target appendage.
    pub effect: AppendageEffect,
    /// The health impact of the action on the target appendage.Positive values heal, negative values damage.
    pub impact: i8,
}

/// Represents a character with a name and body structure
/// A creature is an entity with a pre-defined set of components.
#[derive(Debug, Clone)]
pub struct Creature {
    pub name: String,
    pub corpus: Appendage,
    pub components: HashMap<String, Appendage>,
}

impl Creature {
    pub fn get_character_health(&self) -> u8 {
        let (appendages, total_health) = self.calculate_corpus_health(self.corpus.clone());
        let average_health = if appendages > 0 {
            total_health / (appendages)
        } else {
            0
        };
        average_health as u8
    }

    fn calculate_corpus_health(&self, appendage: Appendage) -> (i32, i32) {
        if appendage.connected_to.is_none() {
            return (1, appendage.get_health().into());
        }
        let mut total_appendages = 1i32; // Count the current appendage
        let mut total_health = appendage.get_health() as i32;
        if let Some(children) = &appendage.connected_to {
            for child in children {
                let (child_count, child_health) = self.calculate_corpus_health(child.clone());
                total_appendages += child_count;
                total_health += child_health;
            }
        }
        (total_appendages, total_health)
    }

    // traverse the morphology tree and apply the action to the target appendage
    pub(crate) fn apply_action(&mut self, action: CreatureActions) {
        let root = &mut self.corpus;
        Self::get_appendage_ref_by_name(root, action);
    }

    fn get_appendage_ref_by_name(appendage: &mut Appendage, action: CreatureActions) {
        if appendage.name == action.target {
            appendage.apply_effect(action.effect, action.impact);
            return;
        }
        if let Some(ref mut children) = appendage.connected_to {
            for child in children.iter_mut() {
                Self::get_appendage_ref_by_name(child, action.clone());
            }
        }
    }
}
