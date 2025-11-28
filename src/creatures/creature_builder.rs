use std::collections::HashMap;

/// Describes the state of an appendage.
#[derive(Clone, Default, Copy, Debug, Eq, Hash, PartialEq)]
enum AppendageHealth {
    /// Full health of the appendage with no impairments.
    #[default]
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
#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct Appendage {
    /// The name of the appendage (e.g., "Left Arm", "Right Leg")
    pub(crate) name: String,
    /// The state of the appendage based on health.
    state: AppendageHealth,
    /// A log of significant events affecting the appendage.
    /// Stored as a tuple of Effect and the health impact it had on the appendage.
    effect_history: HashMap<AppendageEffect, i8>,
    /// An appendage is connected to something else ( Torso -> Head, leg -> foot, Arm -> Hand )
    pub(crate) connected_to: Option<Vec<Appendage>>,
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

    pub(crate) fn get_health(&self) -> i8 {
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

    pub(crate) fn apply_effect(&mut self, attack_effect: AppendageEffect, health_impact: i8) {
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
            // let mut new_appendage = Appendage::default();
            // new_appendage.name = child_name.to_owned();
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
