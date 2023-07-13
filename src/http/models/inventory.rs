use std::collections::HashMap;

use super::store::Currency;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct InventoryResponse {
    pub items: Vec<InventoryItem>,
    pub definitions: Vec<&'static ItemDefinition>,
}

#[derive(Debug, Serialize)]
pub struct InventoryDefinitions {
    pub total_count: usize,
    pub list: &'static [ItemDefinition],
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

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemDefinition {
    pub name: String,
    pub i18n_name: String,
    pub i18n_description: Option<String>,
    pub loc_name: Option<String>,
    pub loc_description: Option<String>,
    pub custom_attributes: HashMap<String, Value>,
    #[serialize_always]
    pub secret: Option<Value>,
    pub category: String,
    pub attachable_categories: Vec<String>,
    pub rarity: Option<String>,
    pub droppable: Option<bool>,
    pub cap: Option<u32>,
    pub on_consume: Option<Vec<Value>>,
    pub on_add: Option<Vec<Value>>,
    pub on_remove: Option<Vec<Value>>,
    pub restrictions: Option<String>,
    pub default_namespace: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InventoryItem {
    pub item_id: Uuid,
    pub definition_name: String,
    pub stack_size: u32,
    pub seen: bool,
    pub instance_attributes: Value,
    pub created: DateTime<Utc>,
    pub last_grant: DateTime<Utc>,
    pub earnd_by: String,
    pub restricted: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityResult {
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
    pub item_definitions: Vec<&'static ItemDefinition>,
    pub entitlements_granted: Vec<Value>,
    pub prestige_progression_map: Value,
}
