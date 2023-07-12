use axum::{extract::Path, Json};
use hyper::StatusCode;
use log::debug;

use crate::http::models::{
    mission::{FinishMissionRequest, StartMissionRequest, StartMissionResponse},
    RawJson,
};

static CURRENT_MISSIONS_DEFINITION: &str =
    include_str!("../../resources/data/currentMissions.json");

/// GET /mission/current
///
/// Obtains a list of currently avaiable missions
pub async fn current_missions() -> RawJson {
    RawJson(CURRENT_MISSIONS_DEFINITION)
}

/// GET /user/mission/:id
///
/// Obtains the details about a specific mission
pub async fn get_mission(Path(mission_id): Path<u32>) -> RawJson {
    static RESP: &str =
        include_str!("../../resources/defs/raw/Get_Mission_Details-1688700361289.json");
    RawJson(RESP)
}

/// POST /user/mission/:id/start
///
/// Starts a mission
pub async fn start_mission(
    Path(mission_id): Path<u32>,
    Json(req): Json<StartMissionRequest>,
) -> Json<StartMissionResponse> {
    debug!("Mission started: {} {:?}", mission_id, req);

    let res = StartMissionResponse {
        match_id: "60474918".to_string(),
    };
    Json(res)
}

/// POST /user/mission/:id/finish
///
/// Submits the details of a mission that has been finished
pub async fn finish_mission(
    Path(mission_id): Path<u32>,
    Json(req): Json<FinishMissionRequest>,
) -> StatusCode {
    debug!("Mission finished: {} {:?}", mission_id, req);

    StatusCode::NO_CONTENT
}
