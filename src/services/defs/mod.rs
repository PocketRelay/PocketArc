use std::{collections::HashMap, process::exit};

use crate::http::models::{character::SkillDefinition, inventory::ItemDefinition};
use log::{debug, error};
use uuid::Uuid;

/// Definitions for all the items
pub const INVENTORY_DEFINITIONS: &str =
    include_str!("../../resources/data/inventoryDefinitions.json");

pub const SKILL_DEFINITIONS: &str = include_str!("../../resources/data/skillDefinitions.json");

pub struct Definitions {
    pub inventory: LookupList<String, ItemDefinition>,
    pub skills: LookupList<Uuid, SkillDefinition>,
}

impl Definitions {
    pub fn load() -> Self {
        debug!("Loading definitions");

        let inventory = Self::load_inventory();
        let skills = Self::load_skills();

        Self { inventory, skills }
    }

    fn load_inventory() -> LookupList<String, ItemDefinition> {
        let list: Vec<ItemDefinition> = match serde_json::from_str(INVENTORY_DEFINITIONS) {
            Ok(value) => value,
            Err(err) => {
                error!("Failed to load inventory definitions: {}", err);
                exit(1);
            }
        };

        debug!("Loaded {} inventory item definition(s)", list.len());

        let map: HashMap<String, ItemDefinition> = list
            .iter()
            .map(|value| (value.name.clone(), value.clone()))
            .collect();

        LookupList { map, list }
    }

    fn load_skills() -> LookupList<Uuid, SkillDefinition> {
        let list: Vec<SkillDefinition> = match serde_json::from_str(SKILL_DEFINITIONS) {
            Ok(value) => value,
            Err(err) => {
                error!("Failed to load skill definitions: {}", err);
                exit(1);
            }
        };

        debug!("Loaded {} skill definition(s)", list.len());

        let map: HashMap<Uuid, SkillDefinition> = list
            .iter()
            .map(|value| (value.name.clone(), value.clone()))
            .collect();

        LookupList { map, list }
    }
}

/// Map and list combined
pub struct LookupList<K, V> {
    pub map: HashMap<K, V>,
    pub list: Vec<V>,
}