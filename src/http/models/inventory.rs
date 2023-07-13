use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use super::store::Currency;

#[derive(Debug, Serialize)]
pub struct InventoryResponse {
    pub items: Vec<InventoryItem>,
    pub definitions: Vec<&'static ItemDefinition>,
}

#[derive(Debug, Serialize)]

pub struct InventoryDefinitions {
    pub total_count: usize,
    pub list: Vec<&'static ItemDefinition>,
}

#[derive(Debug, Deserialize)]
pub struct InventorySeenList {
    pub list: Vec<Uuid>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InventoryConsumeRequest {
    pub items: Vec<ConsumeTarget>,
    pub namespace: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConsumeTarget {
    pub item_id: Uuid,
    pub target_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemDefinition {
    pub name: String,
    pub i18n_name: String,
    pub i18n_description: String,
    pub loc_name: String,
    pub loc_description: String,
    pub custom_attributes: HashMap<String, Value>,
    pub secret: Option<Value>,
    pub category: String,
    pub attachable_categories: Vec<String>,
    pub rarity: String,
    #[serde(default)]
    pub droppable: Option<bool>,
    #[serde(default)]
    pub cap: Option<u32>,
    #[serde(default)]
    pub on_consume: Option<Vec<Value>>,
    #[serde(default)]
    pub on_add: Option<Vec<Value>>,
    #[serde(default)]
    pub on_remove: Option<Vec<Value>>,
    #[serde(default)]
    pub restrictions: Option<String>,
    pub default_namespace: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InventoryItem {
    pub item_id: Uuid,
    pub definition_name: String,
    pub stack_size: u32,
    pub seen: bool,
    pub instance_attributes: Vec<Value>,
    pub created: DateTime<Utc>,
    pub last_grant: DateTime<Utc>,
    pub earned_by: String,
    pub restricted: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InventoryConsumeResponse {
    pub previous_xp: u32,
    pub xp: u32,
    pub xp_gained: u32,
    pub previous_level: u32,
    pub level: u32,
    pub level_up: bool,
    pub challenges_updated_count: u32,
    pub challenges_completed_count: u32,
    pub challenges_updated: Vec<Value>,
    pub updated_challenge_ids: Vec<Value>,
    pub news_triggered: u32,
    pub currencies: Vec<Currency>,
    pub currency_earned: Vec<Currency>,
    pub items_earned: Vec<InventoryItem>,
    pub item_definitions: Vec<ItemDefinition>,
    pub entitlements_granted: Vec<Value>,
    pub prestige_progression_map: Value,
}
