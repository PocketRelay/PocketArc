use axum::{extract::Path, Json};
use serde_json::Map;
use uuid::{uuid, Uuid};

use crate::{
    http::models::leaderboard::{
        LeaderboardCategory, LeaderboardIdent, LeaderboardResponse, LeaderboardRow,
        LeaderboardsResponse,
    },
    utils::models::LocaleNameWithDesc,
};

/// GET /leaderboards
///
/// Retrieves a list of the leaderboard categories
pub async fn get_leaderboards() -> Json<LeaderboardsResponse> {
    Json(LeaderboardsResponse {
        total_count: 2,
        list: vec![
            LeaderboardCategory {
                name: uuid!("2e9181f0-bd7b-e489-1a64-91598df0780c"),

                stat_collection_name: uuid!("3e02497e-9c2b-6ed6-0dfb-028c134326dc"),
                stat_owner_name: "personaId".to_string(),
                ranked_stat_name: "pathfinderRating".to_string(),
                i18n_ranked_stat: "".to_string(),
                seconds_to_live_after_last_write: 2678400,
                properties: vec![],
                owner_id_type: "User".to_string(),
                locale: LocaleNameWithDesc {
                    i18n_name: "114564".to_string(),
                    i18n_description: Some("".to_string()),
                    loc_name: Some("APEX".to_string()),
                    loc_description: None,
                },
            },
            LeaderboardCategory {
                name: uuid!("aff90bf0-a9fd-0a5e-679d-60fc8691ff45"),
                stat_collection_name: uuid!("b3b3061a-6056-fb0a-7edd-7a09b8c90650"),
                stat_owner_name: "personaId".to_string(),
                ranked_stat_name: "challengeRating".to_string(),
                i18n_ranked_stat: "".to_string(),
                seconds_to_live_after_last_write: 2678400,
                properties: vec![],
                owner_id_type: "User".to_string(),

                locale: LocaleNameWithDesc {
                    i18n_name: "114565".to_string(),
                    i18n_description: Some("".to_string()),
                    loc_name: Some("CHALLENGE".to_string()),
                    loc_description: None,
                },
            },
        ],
    })
}

/// GET /leaderboards/:id
///
/// Retrieves the contents of a specific leaderboard
pub async fn get_leaderboard(Path(name): Path<Uuid>) -> Json<LeaderboardResponse> {
    Json(LeaderboardResponse {
        identifier: LeaderboardIdent {
            name,
            property_value_map: Map::new(),
        },
        rows: vec![LeaderboardRow {
            rank: 1,
            name: "Jacobtread".to_string(),
            owner_id: 1,
            stat_value: 512.0,
        }],
    })
}
