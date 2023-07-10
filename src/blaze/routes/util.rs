use std::time::{SystemTime, UNIX_EPOCH};

use blaze_pk::types::TdfMap;

use crate::blaze::{
    models::util::{
        ClientConfigRequest, ClientConfigResponse, PingResponse, PostAuthResponse, PreAuthResponse,
    },
    session::Session,
};

use super::EmptyData;

pub async fn pre_auth(session: &mut Session, _req: EmptyData) -> PreAuthResponse {
    PreAuthResponse {
        target: session.host_target.clone(),
    }
}

pub async fn post_auth(_session: &mut Session, _req: EmptyData) -> PostAuthResponse {
    PostAuthResponse
}

pub async fn fetch_client_config(
    _session: &mut Session,
    req: ClientConfigRequest,
) -> ClientConfigResponse {
    let config: TdfMap<String, String> = match req.id.as_str() {
        "IdentityParams" => [
            ("display", "console2/welcome"),
            ("redirect_uri", "http://127.0.0.1/success"),
        ]
        .into_iter()
        .collect(),
        _ => TdfMap::new(),
    };

    ClientConfigResponse { config }
}

pub async fn ping(_session: &mut Session, _req: EmptyData) -> PingResponse {
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();
    PingResponse { time }
}
